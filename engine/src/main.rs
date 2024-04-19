use std::env;
use engine::game;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RAYON_NUM_THREADS", "8");

    // Get game settings from user
    let mut input_human_players = String::new();
    println!("Enter the amount of human players in this game.");
    std::io::stdin().read_line(&mut input_human_players).unwrap();
    let input = input_human_players.trim();
    let human_players = match input.parse::<u8>() {
        Ok(n) => {
            if n > 2 {
                println!("Error: Maximum of 2 human players allowed.");
                return;
            }
            println!("Setting human players to {}.", n);
            n
        },
        Err(_) => {
            println!("Error: Invalid input. Please enter a number from 0 to 2.");
            return;
        }
    };

    // Get the depth of the search algorithm
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
    game::main_game_loop(human_players, depth);

    // Wait for the user to press enter before closing the program
    let mut input = String::new();
    println!("Press enter to close the game.");
    std::io::stdin().read_line(&mut input).unwrap();

}

#[cfg(test)]
mod tests {
    use types::{
        position::Position,
        square::Square,
    };
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