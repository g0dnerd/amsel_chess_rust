use types::position::Position;
use types::square::Square;
use rand::seq::SliceRandom;
use crate::game;
use crate::evaluation;
use crate::movegen;

// The NegaMax algorithm is a variant of the minimax algorithm that is used to find the best move in a two-player, zero-sum game.

const MAX_DEPTH: i32 = 6;

// Returns all legal moves for the current position ordered by rough likelihood of being played
fn order_moves(mut moves: Vec<(Square, Square)>, pos: &mut Position) -> Vec<(Square, Square)> {
    moves.shuffle(&mut rand::thread_rng());
    let mut ordered_moves = Vec::new();
    if moves.len() < 2 {
        return moves;
    } else {
        for (start, end) in moves.iter() {
            // First, order by captures
            if pos.is_capture(end) {
                ordered_moves.push((*start, *end));
            }
            // Then, order by check
            if game::is_check(pos, start, end) {
                ordered_moves.push((*start, *end));
            }
            // Then, order by promotion
            if pos.is_promotion(start, end) {
                ordered_moves.push((*start, *end));
            }
            // Add the rest
            ordered_moves.push((*start, *end));
        }
    }
    ordered_moves
}

fn alphabeta(pos: &mut Position, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 || !pos.state.game_result.is_ongoing() {
        return evaluation::main_evaluation(pos) as i32;
    }
    let mut legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    legal_moves = order_moves(legal_moves, pos);
    for (square, target_square) in legal_moves.iter_mut() {
        let mut new_pos = pos.clone();
        new_pos.make_move(&square, &target_square);
        println!("Evaluating move: {:?} {:?}", square, target_square);
        let score = alphabeta(&mut new_pos, depth - 1, beta, alpha);
        alpha = alpha.max(score);
        if alpha >= beta {
            println!("Pruning line {:?} {:?}", square, target_square);
            return alpha;
        }
    }
    alpha
}

pub fn find_best_move(pos: &mut Position) -> (Square, Square) {
    println!("Looking for best move for {:?}.", pos.state.active_player);
    let mut best_move = (Square::A1, Square::A1);
    let legal_moves = order_moves(movegen::get_all_legal_moves_for_color(pos.state.active_player, pos), pos);
    let mut alpha = i32::MIN;
    let beta = i32::MAX;

    if legal_moves.len() == 1 {
        return legal_moves[0];
    }

    for (square, target_square) in legal_moves.iter() {
        println!("Looking at root move {:?} {:?}", square, target_square);
        let mut new_pos = pos.clone();
        new_pos.make_move(&square, &target_square);
        let score = alphabeta(&mut new_pos, MAX_DEPTH as u8, alpha, beta);
        if score > alpha {
            alpha = score;
            best_move = (*square, *target_square);
        }
    }
    best_move
}
