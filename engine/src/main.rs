use std::env;
use types::position::Position;
use types::square::Square;
use engine::{movegen, game, parse_input};
use rand::seq::SliceRandom;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Main CLI Loop
    let mut pos = Position::new();
    
     while pos.state.game_result.is_ongoing() {
        pos.print_position();

        println!("It is now {:?}'s turn.", pos.state.active_player);
        /* match pos.state.active_player {
            types::Color::White => println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_black.squares_from_bb()),
            types::Color::Black => println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_white.squares_from_bb()),
        } */

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

        let move_legality = game::is_legal_move(square, target_square, &pos);
        match move_legality {
            Ok(_) => {
                pos.make_move(&square, &target_square);
                game::update_attacked_squares(&mut pos);
            },
            Err(e) => {
                println!("Illegal move: {}", e);
                continue;
            }
        }

        // Make a random move for the AI

        println!("It is now {:?}'s turn.", pos.state.active_player);
        /* match pos.state.active_player {
            types::Color::White => println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_black.squares_from_bb()),
            types::Color::Black => println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_white.squares_from_bb()),
        } */

        let legal_moves = 
            movegen::get_all_legal_moves_for_color(pos.state.active_player, &pos);

        let squares: Vec<Square> = legal_moves
            .iter()
            .filter_map(|(square, moves)|
                if !moves.is_empty() { Some(*square) } else { None})
            .collect();
        if legal_moves.is_empty() {
            match pos.state.active_player {
                types::Color::White => {
                    let king_square = (pos.piece_boards[4] & pos.color_bitboards[0]).squares_from_bb()[0];
                    if pos.attacked_by_black.contains(king_square) {
                        println!("Black wins by checkmate.");
                        pos.print_position();
                        println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_black.squares_from_bb());
                        pos.state.game_result = types::state::GameResult(types::Results::BLACK_VICTORY);
                    } else {
                        println!("Stalemate.");
                        pos.print_position();
                        pos.state.game_result = types::state::GameResult(types::Results::STALEMATE);
                    }
                },
                types::Color::Black => {
                    let king_square = (pos.piece_boards[4] & pos.color_bitboards[1]).squares_from_bb()[0];
                    if pos.attacked_by_white.contains(king_square) {
                        println!("White wins by checkmate.");
                        println!("Squares that are attacked by opponent: {:?}", pos.attacked_by_white.squares_from_bb());
                        pos.print_position();
                        pos.state.game_result = types::state::GameResult(types::Results::WHITE_VICTORY);
                    } else {
                        println!("Stalemate.");
                        pos.print_position();
                        pos.state.game_result = types::state::GameResult(types::Results::STALEMATE);
                    }
                }
            }
        }
        let mut rng = rand::thread_rng();
        if let Some(random_square) = squares.choose(&mut rng) {
            let destination_squares = legal_moves.get(random_square).unwrap().squares_from_bb();
            if let Some(target_square) = destination_squares.choose(&mut rng) {
                pos.make_move(random_square, target_square);
                println!("AI move: {:?} {:?}", random_square, target_square);
                game::update_attacked_squares(&mut pos);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use types::bitboard::BitBoard;

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

}