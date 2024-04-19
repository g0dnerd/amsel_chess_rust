use std::sync::atomic::AtomicBool;
use std::{cmp,
    time::Instant,
    collections::HashMap,
    sync::Mutex,
};
use rayon::prelude::*;
use rand::seq::SliceRandom;
use lazy_static::lazy_static;
use indicatif::{ProgressBar, ProgressStyle};
use crate::movegen;
use crate::game;
use crate::evaluation;
use types::square::Square;
use types::position::Position;
use precompute::rng;

const NUM_PIECE_TYPES: usize = 12;
const NUM_SQUARES: usize = 64;

// Define the Zobrist keys as a global variable
lazy_static! {
    static ref ZOBRIST_KEYS: [[u64; NUM_SQUARES]; NUM_PIECE_TYPES] = initialize_zobrist_keys();
}

// Define the transposition table as a global variable
lazy_static! {
    static ref TRANSPOSITION_TABLE: Mutex<HashMap<u64, TranspositionEntry>> = Mutex::new(HashMap::new());
}

lazy_static! {
    static ref MATE_IN_ONE_FOUND: AtomicBool = AtomicBool::new(false);
}

#[derive(Debug, Copy, Clone)]
struct TranspositionEntry {
    depth: u8,
    score: i32,
}

struct SearchResult {
    score: i32,
    best_move: (Square, Square),
}

struct SearchParameters{
    alpha: i32,
    beta: i32,
    depth: u8,
}


// Function to initialize the Zobrist keys
fn initialize_zobrist_keys() -> [[u64; NUM_SQUARES]; NUM_PIECE_TYPES] {
    let mut rng_instance = rng::Rng::default();
    let mut keys = [[0; NUM_SQUARES]; NUM_PIECE_TYPES];
    let mut dupe_keys = Vec::new();
    for piece_type in 0..NUM_PIECE_TYPES {
        for square in 0..NUM_SQUARES {
            let key = rng_instance.next_u64();
            // println!("Generated key: {} for piece type {} and square {}", key, piece_type, square);
            if dupe_keys.contains(&key) {
                panic!("Duplicate key generated: {}", key);
            }
            dupe_keys.push(key);
            keys[piece_type][square] = key;
        }
    }
    // println!("Successfully initialized Zobrist keys.");
    keys
}

fn calculate_hash(pos: &Position) -> u64 {
    let mut hash = 0;
    for square in 0..NUM_SQUARES {
        if let Some((piece, _color)) = pos.piece_at(Square::index(square)) {
            hash ^= ZOBRIST_KEYS[piece as usize][square];
        }
    }
    // println!("Calculated hash: {}", hash);
    hash
}

// Function to get a position's entry from the transposition table
fn get_entry(hash: u64) -> Option<TranspositionEntry> {
    // println!("Attempting to get entry for hash {} from transposition table.", hash);
    match TRANSPOSITION_TABLE.lock() {
        Ok(table) => {
            // println!("Acquired lock on transposition table at line {}, getting entry.", line!());
            let entry = table.get(&hash).cloned();
            // println!("Released lock on transposition table at line {}.", line!());
            entry
        },
        Err(e) => {
            println!("Error acquiring lock on transposition table while getting entry: {:?}", e);
            None
        }
    }
}

// Function to store a position's entry in the transposition table
fn store_entry(hash: u64, entry: TranspositionEntry) {
    // println!("Attempting to store entry for hash {} in transposition table.", hash);
    match TRANSPOSITION_TABLE.lock() {
        Ok(mut table) => {
            // println!("Acquired lock on transposition table, storing entry.");
            // Check for hash collision
            if let Some(old_entry) = table.get(&hash) {
                if entry.depth > old_entry.depth {
                    // println!("During hash collision, replacing entry because new entry has depth {} and old entry has depth {}.", entry.depth, old_entry.depth);
                    table.insert(hash, entry);
                } else {
                    return;
                }
            } else {
                table.insert(hash, entry);
            }
            // println!("Released lock on transposition table.");
        },
        Err(_) => {
            println!("Error acquiring lock on transposition table while storing entry.");
        }
    }
}

// Returns all legal moves for the current position ordered by rough likelihood of being played
fn order_moves(mut moves: Vec<(Square, Square)>, pos: &mut Position) -> Vec<(Square, Square)> {
    moves.shuffle(&mut rand::thread_rng());
    moves.sort_by_key(|&(start, end)| {
        match () {
            () if game::would_give_check(pos, &start, &end) => 0, 
            () if pos.is_promotion(&start, &end) => 1,
            () if pos.is_capture(&end) => 2,
            _ => 3,
        }
    });
    moves
}

fn negamax(pos: &mut Position, params: &mut SearchParameters) -> i32 {
    
    // println!("Negamax called for line {:?}", pos.move_history);

    // If the position has already been evaluated to the desired depth, return the stored score
    let hash = calculate_hash(pos);

    if let Some(entry) = get_entry(hash) {
        if entry.depth >= params.depth {
            return entry.score;
        }
    } 

    if MATE_IN_ONE_FOUND.load(std::sync::atomic::Ordering::Relaxed) {
        return i32::MIN;
    }

    if params.depth == 0 || !pos.state.game_result.is_ongoing() {
        return evaluation::main_evaluation(pos);
    }

    // Retrieve and order all legal moves
    let mut legal_moves =
        movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    legal_moves = order_moves(legal_moves, pos);

    let mut score = i32::MIN + 1;
    let mut alpha = params.alpha;

    // Iterate over all legal moves
    for (from, to) in legal_moves.iter() {
        // println!("Evaluating move {:?} -> {:?}", from, to);
        let mut new_pos = pos.clone();
        game::make_specific_engine_move(&mut new_pos, *from, *to);

        score = cmp::max(score, -negamax(&mut new_pos, &mut SearchParameters {
            alpha: -params.beta,
            beta: -alpha,
            depth: params.depth - 1,
        }));
        
        store_entry(hash, TranspositionEntry {
            depth: params.depth,
            score,
        });

        alpha = cmp::max(alpha, score);

        // Beta-cutoff
        if alpha >= params.beta {
            break;
        }

    }
    // Return the best score found (or the cutoff if no improvement was made)
    score

}

pub fn find_best_move(pos: &mut Position, depth: u8) -> (Square, Square) {
    let start_time = Instant::now();

    MATE_IN_ONE_FOUND.store(false, std::sync::atomic::Ordering::Relaxed);

    println!("Running search at depth {} with {} threads", depth, rayon::current_num_threads());
    
    let mut legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    if legal_moves.len() == 1 {
        return legal_moves[0];
    }

    println!("Evaluating {} legal moves", legal_moves.len());

    let bar = ProgressBar::new(legal_moves.len() as u64);
    bar.set_style(ProgressStyle::with_template("Move {pos}/{len} [{bar:40.cyan/blue}] {elapsed_precise}").
        unwrap().
        progress_chars("#>-"));
    bar.inc(0);
    legal_moves = order_moves(legal_moves, pos);

    let alpha = i32::MIN + 1;
    let beta = i32::MAX - 1;

    let results: Vec<SearchResult> = legal_moves.par_iter().
        map(|&(from, to)| {
            let mut new_pos = pos.clone();
            game::make_specific_engine_move(&mut new_pos, from, to);
            if game::is_in_checkmate(&mut new_pos) {
                MATE_IN_ONE_FOUND.store(true, std::sync::atomic::Ordering::Relaxed);
                return SearchResult {
                    score: i32::MAX,
                    best_move: (from, to),
                };
            }
            let score = -negamax(&mut new_pos, &mut SearchParameters {
                alpha,
                beta,
                depth,
            });

            bar.inc(1);
            SearchResult {
                score,
                best_move: (from, to),
            }
        }).collect();
    let best_result = results.into_iter().max_by_key(|r| r.score).unwrap_or(SearchResult {
        score: i32::MIN,
        best_move: (Square::A1, Square::A1),
    });
    
    bar.finish();
    let duration = start_time.elapsed();
    println!("Search completed in {} seconds", duration.as_secs_f32());

    best_result.best_move
}