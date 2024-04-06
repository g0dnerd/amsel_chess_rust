extern crate amsel_chess_rust;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let test_pos = amsel_chess_rust::position::Position::new();

    // Tests the board's print method
    test_pos.print_position();

    // Tests the legal moves for the knight on b1 and prints them
    let square_b1 = amsel_chess_rust::Square::new(1); 
    let knight_b1_moves = test_pos.get_legal_moves_by_square(square_b1);
    let knight_b1_moves = knight_b1_moves.squares_from_bb();
    println!("Knight on b1 has {} legal moves", knight_b1_moves.len());
    for i in knight_b1_moves.iter() {
        println!("{}", amsel_chess_rust::get_printable_square_from_index(i));
    }

    // Tests the legal moves for the bishop on c1 and prints them
    let square_c1 = amsel_chess_rust::Square::new(2); 
    let bishop_c1_moves = test_pos.get_legal_moves_by_square(square_c1);
    let bishop_c1_moves = bishop_c1_moves.squares_from_bb();
    println!("Bishop on c1 has {} legal moves", bishop_c1_moves.len());
    for i in bishop_c1_moves.iter() {
        println!("{}", amsel_chess_rust::get_printable_square_from_index(i));
    }

    // Tests the legal moves for the rook on a1 and prints them
    let square_a1 = amsel_chess_rust::Square::new(0);
    let rook_a1_moves = test_pos.get_legal_moves_by_square(square_a1);
    let rook_a1_moves = rook_a1_moves.squares_from_bb();
    println!("Rook on a1 has {} legal moves", rook_a1_moves.len());
    for i in rook_a1_moves.iter() {
        println!("{}", amsel_chess_rust::get_printable_square_from_index(i));
    }

    // Tests the legal moves for the queen on d1 and prints them
    let square_d1 = amsel_chess_rust::Square::new(3);
    let queen_d1_moves = test_pos.get_legal_moves_by_square(square_d1);
    let queen_d1_moves = queen_d1_moves.squares_from_bb();
    println!("Queen on d1 has {} legal moves", queen_d1_moves.len());
    for i in queen_d1_moves.iter() {
        println!("{}", amsel_chess_rust::get_printable_square_from_index(i));
    }

    // Tests the magics precomputation
    let mut rng = amsel_chess_rust::rng::Rng::default();
    amsel_chess_rust::precompute::precompute_magics(&amsel_chess_rust::precompute::ROOK, "Rook", &mut rng);
}