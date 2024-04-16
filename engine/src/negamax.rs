use std::cmp;
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use crate::movegen;
use crate::game;
use crate::evaluation;
use types::square::Square;
use types::position::Position;

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

fn negamax(pos: &mut Position, depth: u8, alpha: i32, beta: i32) -> i32 {
    if depth == 0 || !pos.state.game_result.is_ongoing() {
        return evaluation::main_evaluation(pos);
    }

    // Retrieve and order all legal moves
    let mut legal_moves =
        movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    legal_moves = order_moves(legal_moves, pos);

    // Initialize the maximum score to negative infinity
    // let mut local_best_score = i32::MIN + 1;
    let mut alpha = alpha;

    // Iterate over all legal moves
    for (from, to) in legal_moves.iter() {
        let mut new_pos = pos.clone();
        game::make_specific_engine_move(&mut new_pos, *from, *to);

        let score =
            -negamax(&mut new_pos, depth - 1, -beta, -alpha);

        // Beta-cutoff
        if score >= beta {
            return beta;
        }

        // Update the local best score
        alpha = cmp::max(alpha, score);
    }
    // Return the best score found (or the cutoff if no improvement was made)
    alpha

}

pub fn find_best_move(pos: &mut Position, depth: u8) -> (Square, Square) {
    println!("Running search at depth {}", depth);
    
    let mut legal_moves = movegen::get_all_legal_moves_for_color(pos.state.active_player, pos);
    legal_moves = order_moves(legal_moves, pos);
    let mut best_move = (Square::A1, Square::A1);
    let mut best_score = i32::MIN + 1;
    let mut alpha = i32::MIN + 1;
    let beta = i32::MAX - 1;

    // Set up the indicatif progress bar
    let bar = ProgressBar::new(legal_moves.len() as u64);
    bar.set_style(ProgressStyle::with_template("{msg}Move {pos}/{len} [{bar:40.cyan/blue}] {elapsed_precise}").
        unwrap().
        progress_chars("#>-"));

    let mut from_prev = Square::A1;
    let mut to_prev = Square::A1;
    let mut score_prev = i32::MIN + 1;

    for (from, to) in legal_moves.iter() {

        // Update the progress bar
        if from_prev == Square::A1 {
            bar.set_message(format!(
                "Now evaluating move {:?} -> {:?}\n",
                from, to));
        } else {
            bar.set_message(format!(
                "Current best move: {:?} -> {:?} (Score {})\nLast move {:?} -> {:?} had score {}\nNow evaluating move {:?} -> {:?}\n",
                best_move.0, best_move.1, best_score, from_prev, to_prev, score_prev, from, to));
        }

        let mut new_pos = pos.clone();
        game::make_specific_engine_move(&mut new_pos, *from, *to);
        let score = -negamax(&mut new_pos, depth, alpha, beta);

        // Locally keep track of the highest scoring move
        if score > best_score {
            best_score = score;
            best_move = (*from, *to);
        }

        // Update the global cutoff
        alpha = cmp::max(alpha, best_score);
        
        bar.inc(1);
        from_prev = *from;
        to_prev = *to;
        score_prev = score;
    }
    bar.finish();
    best_move
}