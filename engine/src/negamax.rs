use types::position::Position;
use types::square::Square;
use rand::seq::SliceRandom;
use crate::game;
use crate::evaluation;
use crate::movegen;
use std::time::Instant;
use std::cmp;
use std::thread;
use std::sync::{Arc, Mutex};

// The NegaMax algorithm is a variant of the minimax algorithm that is used to find the best move in a two-player, zero-sum game.

pub const MAX_DEPTH: u8 = 4;
const NUM_THREADS: usize = 4;

// The SearchResult struct is used to store the results of a search thread
#[derive(Debug, Copy, Clone)]
struct SearchResult {
    score: i32,
    best_move: (Square, Square),
}

// The SearchParameters struct is used to hold the parameters for a search thread
struct SearchParameters {
    alpha: i32,
    beta: i32,
    depth: u8,
    maximizing: bool,
}

// Returns all legal moves for the current position ordered by rough likelihood of being played
fn order_moves(mut moves: Vec<(Square, Square)>, pos: &mut Position) -> Vec<(Square, Square)> {
    moves.shuffle(&mut rand::thread_rng());
    moves.sort_by_key(|&(start, end)| {
        match () {
            () if game::would_give_check(pos, &start, &end) => 0, 
            () if pos.is_capture(&end) => 1,
            () if pos.is_promotion(&start, &end) => 2,
            _ => 3,
        }
    });
    moves
}

fn alphabeta(pos: &mut Position, params: &mut SearchParameters) -> SearchResult {
    if params.depth == 0 || !pos.state.game_result.is_ongoing() {
        return SearchResult {
            score: evaluation::main_evaluation(pos),
            best_move: (Square::A1, Square::A1),
        };
    }

    let legal_moves = order_moves(movegen::get_all_legal_moves_for_color(pos.state.active_player, pos), pos);
    if params.maximizing {
        let mut score = i32::MIN + 1;
        for (square, target_square) in legal_moves.iter() {
            let mut new_pos = pos.clone();
            game::make_specific_engine_move(&mut new_pos, *square, *target_square);
            // println!("Looking at line {:?}", new_pos.move_history);

            let result = alphabeta(&mut new_pos, &mut SearchParameters {
                alpha: params.alpha,
                beta: params.beta,
                depth: params.depth - 1,
                maximizing: false,
            });
            score = cmp::max(score, result.score);
            if score >= params.beta {
                break;
            }
            params.alpha = cmp::max(params.alpha, score);
        }
        SearchResult {
            score,
            best_move: (Square::A1, Square::A1),
        }
    } else {
        let mut score = i32::MAX - 1;
        for (square, target_square) in legal_moves.iter() {
            let mut new_pos = pos.clone();
            game::make_specific_engine_move(&mut new_pos, *square, *target_square);
            // println!("Looking at line {:?}", new_pos.move_history);

            let result = alphabeta(&mut new_pos, &mut SearchParameters {
                alpha: params.alpha,
                beta: params.beta,
                depth: params.depth - 1,
                maximizing: true,
            });
            score = cmp::min(score, result.score);
            if params.alpha >= params.beta {
                break;
            }
            params.beta = cmp::min(params.beta, score);
        }
        SearchResult {
            score,
            best_move: (Square::A1, Square::A1),
        }
    }
}

pub fn find_best_move(pos: &mut Position) -> (Square, Square) {
    let start_time = Instant::now();
    let legal_moves = order_moves(movegen::get_all_legal_moves_for_color(pos.state.active_player, pos), pos);
    let maximizing_player = match pos.state.active_player {
        types::Color::White => true,
        types::Color::Black => false,
    };
    
    let alpha = i32::MIN + 1;
    let beta = i32::MAX - 1;    

    if legal_moves.len() == 1 {
        return legal_moves[0];
    }

    let chunk_size = cmp::max(legal_moves.len() / NUM_THREADS, 1);
    let chunks: Vec<Vec<(Square, Square)>> = legal_moves.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();

    let mut threads = vec![];
    let shared_pos = Arc::new(Mutex::new(pos.clone()));
    let shared_alpha = Arc::new(Mutex::new(alpha));
    let shared_beta = Arc::new(Mutex::new(beta));

    for chunk in chunks {
        let shared_pos = Arc::clone(&shared_pos);
        let shared_alpha = Arc::clone(&shared_alpha);
        let shared_beta = Arc::clone(&shared_beta);
        let handle = thread::spawn(move || {
            let mut local_best_move = (Square::A1, Square::A1);
            let mut local_alpha = shared_alpha.lock().unwrap();
            let mut local_beta = shared_beta.lock().unwrap();

            for (square, target_square) in &chunk {
                let mut new_pos = shared_pos.lock().unwrap().clone();
                game::make_specific_engine_move(&mut new_pos, *square, *target_square);

                let mut search_params = SearchParameters {
                    alpha: *local_alpha,
                    beta: *local_beta,
                    depth: MAX_DEPTH,
                    maximizing: !maximizing_player,
                };

                let result = alphabeta(&mut new_pos, &mut search_params);

                match maximizing_player {
                    true => {
                        if result.score > *local_alpha {
                            *local_alpha = result.score;
                            local_best_move = (*square, *target_square);
                        }
                    },
                    false => {
                        if result.score < *local_beta {
                            *local_beta = result.score;
                            local_best_move = (*square, *target_square);
                        }
                    },
                }

                match maximizing_player {
                    true => {
                        if *local_alpha >= *local_beta {
                            break;
                        }
                    },
                    false => {
                        if *local_beta <= *local_alpha {
                            break;
                        }
                    },
                }
            }

            SearchResult {
                score: if maximizing_player { *local_alpha } else { *local_beta },
                best_move: local_best_move,
            }
        });

        threads.push(handle);
    }
    
    // Collect Results from threads
    let mut best_move = SearchResult {
        score: i32::MIN,
        best_move: (Square::A1, Square::A1),
    };

    for handle in threads {
        match handle.join() {
            Ok(result) => {
                if result.score > best_move.score {
                    best_move = result;
                }
            },
            Err(e) => {
                println!("Error joining thread, {e:?}");
            }
        }
        
    }

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Time to find engine move at depth {}: {:?}", MAX_DEPTH, elapsed_time);
    best_move.best_move
}

