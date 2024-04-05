extern crate amsel_chess_rust;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let test_pos = amsel_chess_rust::position::init_bitboards();

    /* println!("Bitboards initialized");
    println!("White bitboard: {:?}", test_pos.color_bitboards[0]);
    println!("Black bitboard: {:?}", test_pos.color_bitboards[1]);
    println!("Is there a piece on a8? {}", test_pos.color_bitboards[0].0.bit(7)); */

    amsel_chess_rust::position::print_position(test_pos);
}