use std::env;
use types::position::Position;
use types::square::Square;
use engine::{game, parse_input};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Main CLI Loop
    let mut pos = Position::new();

    while pos.state.game_result.is_ongoing() {
        pos.print_position();

        // Get user input in the format of "a1 a2"
        let mut input = String::new();
        println!("Enter your move: ");
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let input_legality = parse_input::user_input_to_square_index(input);

        match input_legality {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }

        let squares = input_legality.unwrap();
        let square = Square::index(squares[0]);
        let target_square = Square::index(squares[1]);

        let move_legality = game::is_legal_player_move(square, target_square, &pos);
        match move_legality {
            Ok(_) => pos.make_move(square, target_square),
            Err(e) => println!("Illegal move: {}", e),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use types::bitboard::BitBoard;
    use types::Color;
    use engine::game;
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