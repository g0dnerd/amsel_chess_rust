use crate::bitboard::BitBoard;
use crate::square::Square;
use crate::{Color, Castling, Pieces, Results, get_piece_representation};
use crate::state::{State, GameResult};

/* A position contains the minimum amount of information necessary
/ for the engine to calculate moves and evaluate the board state. */ 

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    // Array of two BitBoards, one for each side
    pub color_bitboards: [BitBoard; 2],

    // Array of BitBoards, one for each piece type
    pub piece_boards: [BitBoard; 6],

    pub state: State,

    pub attacked_by_white: BitBoard,
    pub attacked_by_black: BitBoard,
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

            // Initialize attacked squares in the starting position (a3-h3 and a6-h6)
            attacked_by_white: BitBoard::from_u64(0b111111110000000000000000),
            attacked_by_black: BitBoard::from_u64(0b111111110000000000000000000000000000000000000000),
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

    pub fn make_move(&mut self, from: &Square, to: &Square) {
        let (piece, color) = self.piece_at(*from).unwrap();
        // Check for captures and update halfmove counter
        if self.piece_at(*to).is_some() {
            self.state.half_move_counter = 0;
            // Remove the captured piece from the color and piece bitboards
            let (captured_piece, captured_color) = self.piece_at(*to).unwrap();
            let captured_piece_index = match captured_piece {
                Pieces::ROOK => 0,
                Pieces::KNIGHT => 1,
                Pieces::BISHOP => 2,
                Pieces::QUEEN => 3,
                Pieces::KING => 4,
                Pieces::PAWN => 5,
                _ => panic!("Invalid piece"),
            };
            let to_mask = BitBoard::from_square(*to);
            self.color_bitboards[captured_color as usize] ^= to_mask;
            self.piece_boards[captured_piece_index] ^= to_mask;
        } else {
            self.state.half_move_counter += 1;
        }
        
        // Update castling rights
        match piece {
            Pieces::KING => {
                match color {
                    Color::Black => self.state.castling_rights.0 &= !Castling::BLACK_CASTLING,
                    Color::White => self.state.castling_rights.0 &= !Castling::WHITE_CASTLING,
                }
            },
            Pieces::ROOK => {
                match color {
                    Color::Black => {
                        if *from == Square::A8 {
                            self.state.castling_rights.0 &= !Castling::BLACK_QUEEN_SIDE;
                        } else if *from == Square::H8 {
                            self.state.castling_rights.0 &= !Castling::BLACK_KING_SIDE;
                        }
                    },
                    Color::White => {
                        if *from == Square::A1 {
                            self.state.castling_rights.0 &= !Castling::WHITE_QUEEN_SIDE;
                        } else if *from == Square::H1 {
                            self.state.castling_rights.0 &= !Castling::WHITE_KING_SIDE;
                        }
                    },
                }
            },
            Pieces::PAWN => {
                self.state.half_move_counter = 0;
            }
            _ => (),
        }

        let from_mask = BitBoard::from_square(*from);
        let to_mask = BitBoard::from_square(*to);
        let piece_index = match piece {
            Pieces::ROOK => 0,
            Pieces::KNIGHT => 1,
            Pieces::BISHOP => 2,
            Pieces::QUEEN => 3,
            Pieces::KING => 4,
            Pieces::PAWN => 5,
            _ => panic!("Invalid piece"),
        };
        self.color_bitboards[color as usize] ^= from_mask;
        self.color_bitboards[color as usize] |= to_mask;
        self.piece_boards[piece_index] ^= from_mask;
        self.piece_boards[piece_index] |= to_mask;

        self.state.switch_active_player();

        // Check for draw by 50 move rule
        // TODO: only applies if the last move didn't deliver checkmate.
        if self.state.half_move_counter == 100 {
            self.state.game_result = GameResult(Results::DRAW);
        }

        // TODO: update en passant square
    }

    pub fn update_attacked_squares(&mut self, attacked_squares: BitBoard, color: Color) {
        match color {
            Color::White => self.attacked_by_white = attacked_squares,
            Color::Black => self.attacked_by_black = attacked_squares,
        }
    }

}