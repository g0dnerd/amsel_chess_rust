extern crate amsel_chess_rust;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let test_pos = amsel_chess_rust::position::Position::new();

    test_pos.print_position();
}