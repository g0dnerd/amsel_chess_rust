use crate::bitboard::BitBoard;
use crate::square::Square;
use crate::{Color, Pieces, get_piece_representation};
use crate::state::State;

/* A position contains the minimum amount of information necessary
/ for the engine to calculate moves and evaluate the board state. */ 

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    // Array of two BitBoards, one for each side
    pub color_bitboards: [BitBoard; 2],

    // Array of BitBoards, one for each piece type
    pub piece_boards: [BitBoard; 6],

    pub state: State,
}

impl Position {

    pub fn new () -> Position {
        let mut bitboards = [BitBoard::empty(); 2];
        let mut piece_boards = [BitBoard::empty(); 6];

        // Initialize the bitboard for both colors in their starting positions
        // White
        bitboards[0] = BitBoard::from_u64(0b1111111111111111);
        // Black
        bitboards[1] = BitBoard::from_u64(0b1111111111111111000000000000000000000000000000000000000000000000);

        // Initialize the piece bitboards for the respective starting positions
        // Rooks
        piece_boards[0] = BitBoard::from_u64(0b1000000100000000000000000000000000000000000000000000000010000001);
        // Knights
        piece_boards[1] = BitBoard::from_u64(0b100001000000000000000000000000000000000000000000000000001000010);
        // Bishops
        piece_boards[2] = BitBoard::from_u64(0b10010000000000000000000000000000000000000000000000000000100100);
        // Queens
        piece_boards[3] = BitBoard::from_u64(0b100000000000000000000000000000000000000000000000000000001000);
        // Kings
        piece_boards[4] = BitBoard::from_u64(0b1000000000000000000000000000000000000000000000000000000010000);
        // Pawns
        piece_boards[5] = BitBoard::from_u64(0b11111111000000000000000000000000000000001111111100000000);

        // Check if the color bitboards match the piece bitboards
        assert_eq!(bitboards[0] | bitboards[1],
            piece_boards[0] | piece_boards[1] | piece_boards[2] | piece_boards[3] | piece_boards[4] | piece_boards[5],
            "Inconsistent position initialization. Color bitboards do not match piece bitboards.");

        Self {
            color_bitboards: bitboards,
            piece_boards,
            state: State::new(),
        }    

    }

    // Prints out a visual representation of a given board state.
    pub fn print_position(&self) {
        let mut board = [[0; 8]; 8];
        for square in Square::ALL {
            match self.piece_at(square) {
                Some((piece, color)) => {
                    let x = square as usize % 8;
                    let y = square as usize / 8;
                    board[y][x] = match color {
                        Color::White => piece + 1,
                        Color::Black => piece + 7,
                    };
                },
                None => ()
            }
        }
        println!("---------------");
        for row in board.iter().rev() {
            for square in row.iter() {
                print!("{} ", get_piece_representation(*square as u8));
            }
            println!();
        }
    }

    // Returns the piece at a given square or None if the square is empty
    pub fn piece_at(&self, square: Square) -> Option<(u8, Color)> {
        let index = square as usize;
        let mask: u64 = 1 << index;
        let color_mask = if self.color_bitboards[0].0 & mask != 0 {
            Color::White
        } else {
            Color::Black
        };
        
        let piece = if let Some(piece_index) = 
            (0..=5).find(|&i| self.piece_boards[i].0 & mask != 0) {

            match piece_index {
                0 => Pieces::ROOK,
                1 => Pieces::KNIGHT,
                2 => Pieces::BISHOP,
                3 => Pieces::QUEEN,
                4 => Pieces::KING,
                5 => Pieces::PAWN,
                _ => panic!("Invalid piece index"),
            }
        } else {
            return None;
        };
        
        Some((piece, color_mask))
    }

    pub fn all_pieces(&self) -> BitBoard {
        self.color_bitboards[0] | self.color_bitboards[1]
    }

    // Shows a given move on the board by updating the position, not considering legality
    pub fn simulate_move(&self, from: Square, to: Square) -> Self {
        let mut new_pos = self.clone();
        let (piece, color) = self.piece_at(from).unwrap();
        let piece_index = match piece {
            Pieces::ROOK => 0,
            Pieces::KNIGHT => 1,
            Pieces::BISHOP => 2,
            Pieces::QUEEN => 3,
            Pieces::KING => 4,
            Pieces::PAWN => 5,
            _ => panic!("Invalid piece"),
        };
        let from_mask = BitBoard::from_square(from);
        let to_mask = BitBoard::from_square(to);
        new_pos.color_bitboards[color as usize] ^= from_mask;
        new_pos.color_bitboards[color as usize] |= to_mask;
        new_pos.piece_boards[piece_index] ^= from_mask;
        new_pos.piece_boards[piece_index] |= to_mask;
        new_pos
    }

    pub fn make_move(&mut self, from: Square, to: Square) {
        *self = self.simulate_move(from, to);
        println!("Moved from {:?} to {:?}", from, to);
        self.state.switch_active_player();
        // TODO: Increase move counter, update en passant square, update castling rights
    }
}