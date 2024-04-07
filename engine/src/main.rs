use std::env;

use types::square::Square;
use types::position::Position;
use engine::movegen;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let test_pos = Position::new();

    // Tests the board's print method
    test_pos.print_position();

    // Tests legal moves for a rook on a1 in the initial position
    let square = Square::A1;
    let moves = movegen::get_rook_moves_from_position(square, &test_pos);
    println!("Rook moves for a1 in the initial position: {:?}", moves);

    // Tests legal moves for a bishop on c1 in the initial position
    let square = Square::C1;
    let moves = movegen::get_bishop_moves_from_position(square, &test_pos);
    println!("Bishop moves for c1 in the initial position: {:?}", moves);

}