use std::env;
use types::position::Position;
use types::square::Square;
use engine::movegen;
use engine::game;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    // Tests the board's print method
    // TODO: How can I move this into a unit test?
    let mut test_pos = Position::new();
    test_pos.print_position();

    // Fully go through the move generation and simulation flow
    let square = Square::B1;
    let moves = movegen::get_moves_by_square(square, &test_pos);
    let target_square = moves.squares_from_bb()[0];
    let legality = game::is_legal_move(square, target_square, &test_pos);
    println!("Move from {:?} to {:?} is legal: {}", square, target_square, legality);
    test_pos.make_move(square, target_square);
    test_pos.print_position();

    /* test_pos = test_pos.simulate_move(Square::B1, Square::C3);
    test_pos.print_position();

    test_pos = test_pos.simulate_move(Square::D7, Square::D5);
    test_pos.print_position(); */

}

#[cfg(test)]
mod tests {
    use super::*;
    use types::bitboard::BitBoard;
    use types::Color;
    use engine::game;

    #[test]
    fn moves_rook_b1_initial() {
        let test_pos = Position::new();
        let square = Square::B1;
        let moves = movegen::get_rook_moves_from_position(square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_knight_b1_initial() {
        let test_pos = Position::new();
        let square = Square::B1;
        let moves = movegen::get_knight_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::from_u64(327680));
    }

    #[test]
    fn moves_knight_b8_initial() {
        let test_pos = Position::new();
        let square = Square::B8;
        let moves = movegen::get_knight_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::from_u64(5497558138880));
    }

    #[test]
    fn moves_bishop_c1_initial() {
        let test_pos = Position::new();
        let square = Square::C1;
        let moves = movegen::get_bishop_moves_from_position(square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_queen_d1_initial() {
        let test_pos = Position::new();
        let square = Square::D1;
        let moves = movegen::get_queen_moves_from_position(square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_king_e1_initial() {
        let test_pos = Position::new();
        let square = Square::E1;
        let moves = movegen::get_king_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_pawn_f2_initial() {
        let test_pos = Position::new();
        let square = Square::F2;
        let moves = movegen::get_pawn_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::from_u64(538968064));
    }

    #[test]
    fn moves_pawn_h7_initial() {
        let test_pos = Position::new();
        let square = Square::H7;
        let moves = movegen::get_pawn_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::from_u64(141287244169216));
    }

    /* #[test]
    fn moves_pawn_f6_artificial() {
        let test_pos = Position::new();
        let square = Square::F6;
        let moves = movegen::get_pawn_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::from_u64(22517998136852480));
    } */

    #[test]
    #[should_panic(expected = "called on empty square")]
    fn moves_empty_square() {
        let test_pos = Position::new();
        let square = Square::D4;
        let _moves = (movegen::get_king_moves(square, &test_pos),
                     movegen::get_queen_moves_from_position(square, &test_pos),
                     movegen::get_bishop_moves_from_position(square, &test_pos),
                     movegen::get_knight_moves(square, &test_pos),
                     movegen::get_rook_moves_from_position(square, &test_pos));
    }

    #[test]
    fn check_detection_white_initial() {
        let test_pos = Position::new();
        let attackers = game::get_attackers_on_king(Color::White, test_pos);
        assert_eq!(attackers, None);
    }
    
}