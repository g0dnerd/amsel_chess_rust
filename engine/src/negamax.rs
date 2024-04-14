use types::position::Position;
use types::square::Square;
use rand::seq::SliceRandom;
use crate::game;
use crate::evaluation;
use crate::movegen;
use std::time::Instant;
use std::cmp;

// The NegaMax algorithm is a variant of the minimax algorithm that is used to find the best move in a two-player, zero-sum game.

const MAX_DEPTH: i32 = 4;

// Returns all legal moves for the current position ordered by rough likelihood of being played
fn order_moves(mut moves: Vec<(Square, Square)>, pos: &mut Position) -> Vec<(Square, Square)> {
    moves.shuffle(&mut rand::thread_rng());
    moves.sort_by_key(|&(start, end)| {
        match () {
            () if game::is_check(pos, &start, &end) => 0,
            () if pos.is_capture(&end) => 1,
            () if pos.is_promotion(&start, &end) => 2,
            _ => 3,
        }
    });
    moves
}

fn alphabeta(pos: &mut Position, depth: u8, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
    if depth == 0 || !pos.state.game_result.is_ongoing() {
        let terminal_node_score = evaluation::main_evaluation(pos);
        return terminal_node_score;
    }
    let legal_moves = order_moves(movegen::get_all_legal_moves_for_color(pos.state.active_player, pos), pos);
    if maximizing {
        let mut score = i32::MIN + 1;
        for (square, target_square) in legal_moves.iter() {
            let mut new_pos = pos.clone();
            game::make_specific_engine_move(&mut new_pos, *square, *target_square);
            // println!("Looking at line {:?}", new_pos.move_history);
            score = cmp::max(score, alphabeta(&mut new_pos, depth-1, alpha, beta, false));
            if score >= beta {
                break;
            }
            alpha = cmp::max(alpha, score);
        }
        return score;
    } else {
        let mut score = i32::MAX - 1;
        for (square, target_square) in legal_moves.iter() {
            let mut new_pos = pos.clone();
            game::make_specific_engine_move(&mut new_pos, *square, *target_square);
            // println!("Looking at line {:?}", new_pos.move_history);
            score = cmp::min(score, alphabeta(&mut new_pos, depth - 1, alpha, beta, true));
            if score < alpha {
                break;
            }
            beta = cmp::min(beta, score);
        }
        score
    }
}

pub fn find_best_move(pos: &mut Position) -> (Square, Square) {
    let start_time = Instant::now();
    let mut best_move = (Square::A1, Square::A1);
    let legal_moves = order_moves(movegen::get_all_legal_moves_for_color(pos.state.active_player, pos), pos);
    let maximizing_player = match pos.state.active_player {
        types::Color::White => true,
        types::Color::Black => false,
    };
    
    let mut alpha = i32::MIN + 1;
    let beta = i32::MAX - 1;    

    if legal_moves.len() == 1 {
        return legal_moves[0];
    }

    for (square, target_square) in legal_moves.iter() {
        let mut new_pos = pos.clone();
        game::make_specific_engine_move(&mut new_pos, *square, *target_square);
        let score = match maximizing_player {
            true => alphabeta(&mut new_pos, MAX_DEPTH as u8, alpha, beta, !maximizing_player),
            false => -alphabeta(&mut new_pos, MAX_DEPTH as u8, alpha, beta, !maximizing_player),
        };
        if score == i32::MAX {
            let end_time = Instant::now();
            let elapsed_time = end_time.duration_since(start_time);
            println!("Time to find engine move at depth {}: {:?}", MAX_DEPTH, elapsed_time);
            return (*square, *target_square);
        }
        if score > alpha {
            alpha = score;
            best_move = (*square, *target_square);
        }
    }
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Time to find engine move at depth {}: {:?}", MAX_DEPTH, elapsed_time);
    println!("High score was {}", alpha);
    best_move
}
