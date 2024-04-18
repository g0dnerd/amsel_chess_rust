use std::{cmp,
    thread,
    time::Instant,
    collections::HashMap,
    sync::{Arc, Mutex,
        atomic::{AtomicBool, Ordering}
    },
};
use rand::seq::SliceRandom;
use lazy_static::lazy_static;
use indicatif::{ProgressBar, ProgressStyle};
use crate::movegen;
use crate::game;
use crate::evaluation;
use types::square::Square;
use types::position::Position;
use precompute::rng;

const NUM_THREADS: u8 = 4;
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
    println!("Successfully initialized Zobrist keys.");
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
    // println!("Negamax received following hash for current position: {}", hash);

    if let Some(entry) = get_entry(hash) {
        if entry.depth >= params.depth {
            // println!("Found stored entry with depth {} and score {}, terminating child node.", entry.depth, entry.score);
            return entry.score;
        } else {
            // println!("Found stored entry with depth {} but need depth {}", entry.depth, params.depth);
        }
    } else {
        // println!("No stored entry found.");
    }

    if params.depth == 0 || !pos.state.game_result.is_ongoing() {
        return evaluation::main_evaluation(pos);
    }

    // Retrieve and order all legal moves
    let mut legal_moves =
        movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    legal_moves = order_moves(legal_moves, pos);

    // Initialize the maximum score to negative infinity
    // let mut local_best_score = i32::MIN + 1;
    let mut alpha = params.alpha;

    // Iterate over all legal moves
    for (from, to) in legal_moves.iter() {
        // println!("Evaluating move {:?} -> {:?}", from, to);
        let mut new_pos = pos.clone();
        game::make_specific_engine_move(&mut new_pos, *from, *to);

        let score = -negamax(&mut new_pos, &mut SearchParameters {
            alpha: -params.beta,
            beta: -alpha,
            depth: params.depth - 1,
        });
        
        store_entry(hash, TranspositionEntry {
            depth: params.depth,
            score,
        });

        // Beta-cutoff
        if score >= params.beta {
            return params.beta;
        }

        // Update the local best score
        alpha = cmp::max(alpha, score);
    }
    // Return the best score found (or the cutoff if no improvement was made)
    alpha

}

pub fn find_best_move(pos: &mut Position, depth: u8) -> (Square, Square) {
    let start_time = Instant::now();
    println!("Running search at depth {} with {} threads", depth, NUM_THREADS);
    
    let mut legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    if legal_moves.len() == 1 {
        return legal_moves[0];
    }

    println!("Evaluating {} legal moves", legal_moves.len());

    let bar = ProgressBar::new(legal_moves.len() as u64);
    bar.set_style(ProgressStyle::with_template("Move {pos}/{len} [{bar:40.cyan/blue}] {elapsed_precise}").
        unwrap().
        progress_chars("#>-"));

    legal_moves = order_moves(legal_moves, pos);

    let alpha = i32::MIN + 1;
    let beta = i32::MAX - 1;

    // Divide legal moves into NUM_THREADS amount of chunks
    let chunk_size = cmp::max(legal_moves.len() / NUM_THREADS as usize, 1);
    let chunks: Vec<Vec<(Square, Square)>> = legal_moves.chunks(chunk_size).map(|chunk|chunk.to_vec()).collect();

    let mut threads = vec![];

    // Create shared parameters for the threads
    let mate_in_one_found = Arc::new(AtomicBool::new(false));
    let shared_pos = Arc::new(Mutex::new(pos.clone()));
    let shared_alpha = Arc::new(Mutex::new(alpha));
    let shared_beta = Arc::new(Mutex::new(beta));
    let shared_bar = Arc::new(Mutex::new(bar));

    for chunk in chunks {
        // Create local references to the shared parameters
        let shared_pos = Arc::clone(&shared_pos);
        let shared_alpha = Arc::clone(&shared_alpha);
        let shared_beta = Arc::clone(&shared_beta);
        let shared_mate_in_one_found = Arc::clone(&mate_in_one_found);
        let shared_bar = Arc::clone(&shared_bar);

        // Spawn threads
        let thread = thread::spawn(move || {
            let mut local_best_move = (Square::A1, Square::A1);
            let mut local_best_score = i32::MIN + 1;
            let mut local_alpha = shared_alpha.lock().unwrap();
            let local_beta = shared_beta.lock().unwrap();

            for (from, to) in &chunk {
                // If another thread has found a mate in one, return immediately
                if shared_mate_in_one_found.load(Ordering::Relaxed) {
                    return SearchResult {
                        score: i32::MIN,
                        best_move: (Square::A1, Square::A1),
                    };
                }
                let mut new_pos = shared_pos.lock().unwrap().clone();

                game::make_specific_engine_move(&mut new_pos, *from, *to);

                let score = -negamax(&mut new_pos, &mut SearchParameters {
                    alpha: *local_alpha,
                    beta: *local_beta,
                    depth,
                });

                // If the move is checkmate in 1, tell the other threads to stop and return the move
                if score == i32::MAX {
                    shared_mate_in_one_found.store(true, Ordering::Relaxed);
                    return SearchResult {
                        score: i32::MAX,
                        best_move: (*from, *to),
                    };
                }

                let bar = shared_bar.lock().unwrap();
                bar.inc(1);
                drop(bar);

                if score > local_best_score {
                    local_best_score = score;
                    local_best_move = (*from, *to);
                }

                *local_alpha = cmp::max(*local_alpha, local_best_score);
            }
            SearchResult {
                score: local_best_score,
                best_move: local_best_move,
            }
        });

        threads.push(thread);
    }

    // Collect results from threads
    let mut best_result = SearchResult {
        score: i32::MIN + 1,
        best_move: (Square::A1, Square::A1),
    };

    for thread in threads {
        match thread.join() {
            Ok(result) => {
                if result.score > best_result.score {
                    best_result = result;
                }
            },
            Err(_) => {
                println!("Error joining thread");
            }
        }
    }

    let duration = start_time.elapsed();
    println!("Search took {} seconds", duration.as_secs_f32());

    best_result.best_move
}