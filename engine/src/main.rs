use std::env;
use types::position::Position;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    // Tests the board's print method
    // TODO: How can I move this into a unit test?
    let test_pos = Position::new();
    test_pos.print_position();

}

#[cfg(test)]
mod tests {
    use super::*;
    use types::bitboard::BitBoard;
    use types::square::Square;
    use engine::movegen;

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
}