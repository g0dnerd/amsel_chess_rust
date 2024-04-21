use std::env;
use engine::game;
use types::types_utils::string_from_square;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RAYON_NUM_THREADS", "12");

    // Get game settings from user
    let mut input_human_players = String::new();
    println!("Enter the amount of human players in this game. Amount can be 0 or 1.");
    std::io::stdin().read_line(&mut input_human_players).unwrap();
    let input = input_human_players.trim();
    let human_players = match input.parse::<u8>() {
        Ok(n) => {
            if n > 2 {
                println!("Error: Maximum of 1 human player allowed.");
                return;
            }
            println!("Setting human players to {}.", n);
            n
        },
        Err(_) => {
            println!("Error: Invalid input. Please enter a number from 0 to 1.");
            return;
        }
    };

    let mut input_depth = String::new();
    println!("Enter the depth of the search algorithm. Minimum 1, maximum 8.");
    std::io::stdin().read_line(&mut input_depth).unwrap();
    let input = input_depth.trim();
    let depth = match input.parse::<u8>() {
        Ok(n) => {
            if n > 8 {
                println!("Error: Maximum depth is 8.");
                return;
            }
            println!("Setting depth to {}.", n);
            n
        },
        Err(_) => {
            println!("Error: Invalid input. Please enter a number from 1 to 8.");
            return;
        }
    };

    // Main game loop
    let move_history = game::main_game_loop(human_players, depth);
    for mv in move_history {
        println!("{} {}, ", string_from_square(mv.0), string_from_square(mv.1));
    }

    // Wait for the user to press enter before closing the program
    let mut input = String::new();
    println!("Press enter to close the game.");
    std::io::stdin().read_line(&mut input).unwrap();

}

#[cfg(test)]
mod tests {
    use types::position::Position;
    use engine::movegen;
    use types::bitboard::BitBoard;

    #[test]
    fn colorflip_ranks_1_2() {
        let bb = BitBoard::from_u64(65535);
        let flipped = bb.colorflip();
        assert_eq!(flipped, BitBoard::from_u64(18446462598732840960));
    }

    #[test]
    fn colorflip_ranks_7_8() {
        let bb = BitBoard::from_u64(18446462598732840960);
        let flipped = bb.colorflip();
        assert_eq!(flipped, BitBoard::from_u64(65535));
    }


    #[test]
    fn test_piece_at_for_empty() {
        let square = 16;
        let test_pos = Position::new();
        let piece = test_pos.piece_at(square);
        assert_eq!(piece, None);
    }

    #[test]
    fn negate_min_int() {
        let min_int = i32::MIN + 1;
        assert_eq!(-min_int, i32::MAX);
    }

    #[test]
    fn moves_rook_b1_initial() {
        let test_pos = Position::new();
        let square = 1;
        let moves = movegen::slider_moves(0, square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_knight_b1_initial() {
        let test_pos = Position::new();
        let square = 1;
        let moves = movegen::get_knight_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::from_u64(327680));
    }

    #[test]
    fn moves_knight_b8_initial() {
        let test_pos = Position::new();
        let square = 57;
        let moves = movegen::get_knight_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::from_u64(5497558138880));
    }

    #[test]
    fn moves_bishop_c1_initial() {
        let test_pos = Position::new();
        let square = 2;
        let moves = movegen::slider_moves(2, square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_queen_d1_initial() {
        let test_pos = Position::new();
        let square = 3;
        let moves = movegen::slider_moves(3, square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_king_e1_initial() {
        let test_pos = Position::new();
        let square = 4;
        let moves = movegen::get_king_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::empty());
    }

    #[test]
    fn moves_pawn_f2_initial() {
        let test_pos = Position::new();
        let square = 13;
        let moves = movegen::get_pawn_moves(square, &test_pos);
        assert_eq!(moves, BitBoard::from_u64(538968064));
    }

    #[test]
    fn moves_pawn_h7_initial() {
        let test_pos = Position::new();
        let square = 55;
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
        let square = 27;
        let _moves = (movegen::get_king_moves(square, &test_pos),
                     movegen::slider_moves(0, square, &test_pos),
                     movegen::get_knight_moves(square, &test_pos),
                     movegen::slider_moves(2, square, &test_pos),
                     movegen::slider_moves(3, square, &test_pos),
                        movegen::get_pawn_moves(square, &test_pos));
    }

}