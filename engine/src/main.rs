use std::env;

use types::bitboard::BitBoard;
use types::square::Square;
// use crate::movegen;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let test_pos = types::position::Position::new();

    // Tests the board's print method
    test_pos.print_position();

    // Tests legal moves for a rook on d4 with some blockers on the board
    let square = Square::D4;
    let blockers = BitBoard::from_u64(2286984219592704);

    todo!();

}