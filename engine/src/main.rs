use std::env;
use types::position::Position;
use types::square::Square;
use engine::{game, parse_input, evaluation};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Main CLI Loop
    let mut pos = Position::new();
    
    while pos.state.game_result.is_ongoing() {
        pos.print_position();
        if game::is_in_checkmate(&mut pos) {
            match pos.state.active_player {
                types::Color::White => {
                    println!("Black wins by checkmate!");
                    pos.state.game_result = types::state::GameResult(types::Results::BLACK_VICTORY);
                    continue;
                },
                types::Color::Black => {
                    println!("White wins by checkmate!");
                    pos.state.game_result = types::state::GameResult(types::Results::WHITE_VICTORY);
                    continue;
                }
            }
        }
        /* println!("It is now {:?}'s turn.", pos.state.active_player);
        game::make_engine_move(&mut pos);
        break; */

        // Get user input in the format of "a1 a2"
        let mut input = String::new();
        println!("Enter a valid move, type 'legal' to get a list of valid moves or press enter to have the engine move.");
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Check if the user input is in the correct format
        let input_legality = parse_input::user_input_to_square_index(input);
        match input_legality {
            Ok(o) => {
                if o == [99, 99] {
                    continue;
                } else if o == [98, 98] {
                    println!("Legal moves: {:?}", engine::movegen::get_all_legal_moves_for_color(pos.state.active_player, &mut pos));
                    continue;
                }
                else if o == [97, 97] {
                    println!("Engine is making a move.");
                    game::make_engine_move(&mut pos);
                    continue;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }

        let squares = input_legality.unwrap();
        let square = Square::index(squares[0]);
        let target_square = Square::index(squares[1]);
        
        match game::make_player_move(&mut pos, square, target_square) {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }

        println!("Current evaluation: {}", evaluation::main_evaluation(&mut pos));
    }

    // Wait for the user to press enter before closing the program
    /* let mut input = String::new();
    println!("Press enter to close the game.");
    std::io::stdin().read_line(&mut input).unwrap(); */

}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::movegen;
    use types::bitboard::BitBoard;

    #[test]
    fn negate_min_int() {
        let min_int = i32::MIN + 1;
        assert_eq!(-min_int, i32::MAX);
    }

    #[test]
    fn moves_rook_b1_initial() {
        let test_pos = Position::new();
        let square = Square::B1;
        let moves = movegen::get_rook_moves(square, &test_pos);
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
        let moves = movegen::get_bishop_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_queen_d1_initial() {
        let test_pos = Position::new();
        let square = Square::D1;
        let moves = movegen::get_queen_moves(square, &test_pos);
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
                     movegen::get_queen_moves(square, &test_pos),
                     movegen::get_bishop_moves(square, &test_pos),
                     movegen::get_knight_moves(square, &test_pos),
                     movegen::get_rook_moves(square, &test_pos));
    }

}