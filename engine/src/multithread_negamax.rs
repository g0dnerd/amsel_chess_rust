use std::{
    cmp,
    thread,
    sync::{Arc, Mutex},
};
use rand::seq::SliceRandom;
use crate::movegen;
use crate::game;
use crate::evaluation;
use types::square::Square;
use types::position::Position;

const NUM_THREADS: u8 = 4;

#[derive(Debug, Copy, Clone)]
struct SearchResult {
    score: i32,
    best_move: (Square, Square),
}

struct SearchParameters{
    alpha: Arc<Mutex<i32>>,
    beta: Arc<Mutex<i32>>,
    depth: u8,
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
    if params.depth == 0 || !pos.state.game_result.is_ongoing() {
        let eval = evaluation::main_evaluation(pos);
        println!("Reach leaf node with evaluation {}", eval);
        return eval;
    }

    println!("Negamax called.");

    let mut alpha = params.alpha.try_lock().unwrap();
    let mut beta = params.beta.try_lock().unwrap();

    println!("Mutex locks acquired.");

    *alpha = -*alpha;
    *beta = -*beta;

    println!("Alpha and beta inversed.");

    if -*alpha >= -*beta {
        println!("Pruning at depth {} with alpha {} and beta {}", params.depth, -*alpha, -*beta);
        return -*alpha;
    }

    // Retrieve and order all legal moves
    let mut legal_moves =
        movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    legal_moves = order_moves(legal_moves, pos);

    println!("Moves ordered.");

    // let mut local_best_score = *alpha;

    // Iterate over all legal moves
    for (from, to) in legal_moves.iter() {
        println!("negamax running move {:?} {:?}", from, to);
        let mut new_pos = pos.clone();
        game::make_specific_engine_move(&mut new_pos, *from, *to);

        let new_alpha = Arc::clone(&params.alpha);
        let new_beta = Arc::clone(&params.beta);

        // Recursively call negamax with the new position and alpha = -
        let score = -negamax(&mut new_pos, &mut SearchParameters {
                alpha: new_beta,
                beta: new_alpha,
                depth: params.depth - 1,
            });

        // Beta-cutoff
        if score >= *beta {
            println!("Beta cutoff at depth {} with score {} and beta {}", params.depth, score, *beta);
            return *beta;
        }

        // Update the local best score
        *alpha = cmp::max(*alpha, score);
    }

    // Return the best score found (or the cutoff if no improvement was made)
    *alpha

}

pub fn find_best_move(pos: &mut Position, depth: u8) -> (Square, Square) {
    println!("Running search at depth {}", depth);
    
    let mut legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    legal_moves = order_moves(legal_moves, pos);
    let alpha = i32::MIN + 1;
    let beta = i32::MAX - 1;

    if legal_moves.len() == 1 {
        return legal_moves[1];
    }

    // Divide legal moves into NUM_THREADS amount of chunks
    let chunk_size = cmp::max(legal_moves.len() / NUM_THREADS as usize, 1);
    let chunks: Vec<Vec<(Square, Square)>> = legal_moves.chunks(chunk_size).map(|chunk|chunk.to_vec()).collect();

    let mut threads = vec![];

    // Create shared parameters for the threads
    let shared_pos = Arc::new(Mutex::new(pos.clone()));
    let shared_alpha = Arc::new(Mutex::new(alpha));
    let shared_beta = Arc::new(Mutex::new(beta));

    for chunk in chunks {
        // Create local references to the shared parameters
        let shared_pos = Arc::clone(&shared_pos);
        let shared_alpha = Arc::clone(&shared_alpha);
        let shared_beta = Arc::clone(&shared_beta);

        let thread = thread::spawn(move || {
            let mut local_best_move = (Square::A1, Square::A1);
            let mut local_best_score = i32::MIN + 1;
            let mut local_alpha = shared_alpha.lock().unwrap();
            let mut local_beta = shared_beta.lock().unwrap();
            *local_alpha = -*local_alpha;
            *local_beta = -*local_beta;

            for (from, to) in &chunk {
                println!("Thread running move {:?} {:?}", from, to);
                let mut new_pos = shared_pos.lock().unwrap().clone();

                game::make_specific_engine_move(&mut new_pos, *from, *to);

                let new_alpha = Arc::clone(&shared_alpha);
                let new_beta = Arc::clone(&shared_alpha);

                let mut search_params = SearchParameters {
                    alpha: new_alpha,
                    beta: new_beta,
                    depth,
                };
                
                let score = -negamax(&mut new_pos, &mut search_params);

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
                    best_result.score = result.score;
                    best_result.best_move = result.best_move;
                }
            },
            Err(e) => {
                println!("Error joining threads: {e:?}");
            }
        }
    }

    best_result.best_move
}