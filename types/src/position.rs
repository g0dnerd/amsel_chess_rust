use core::panic;

use crate::{
    state::{State, GameResult},
    bitboard::BitBoard,
    types_utils::string_from_square,
    Color,
    Castling,
    Results,
    Piece,
    get_piece_representation,
};

/* A position contains the minimum amount of information necessary
/ for the engine to calculate moves and evaluate the board state. */ 

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    // Array of two BitBoards, one for each side
    pub color_bitboards: [BitBoard; 2],

    // Array of BitBoards, one for each piece type
    pub piece_bitboards: [BitBoard; 6],

    pub state: State,

    // attack bitboards for each square, contains all squares that are attacked by a given square
    pub attack_bitboards: [BitBoard; 64],
    pub attacked_by_white: BitBoard,
    pub attacked_by_black: BitBoard,

    pub move_history: Vec<(u8, u8)>,

    pub en_passant_square: Option<u8>,

   /*  was_last_move_capture: Option<u8>,
    castling_rights_history: Vec<CastlingRights>,
    halfmove_clock_history: Vec<u8>,
    pub piece_giving_check: Option<Square>, */
    pub check: bool,
}

impl Position {

    pub fn new () -> Position {
        let mut bitboards = [BitBoard::empty(); 2];
        let mut piece_boards = [BitBoard::empty(); 6];
        let mut attacks = [BitBoard::empty(); 64];

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

        // Initialize the attack bitboards for each square
        // White knights
        attacks[1] = BitBoard::from_u64(0b1010000000000000000);
        attacks[6] = BitBoard::from_u64(0b101000000000000000000000);

        // White pawns
        attacks[8] = BitBoard::from_u64(0b110000000000000000);
        attacks[9] = BitBoard::from_u64(0b1010000000000000000);
        attacks[10] = BitBoard::from_u64(0b10100000000000000000);
        attacks[11] = BitBoard::from_u64(0b101000000000000000000);
        attacks[12] = BitBoard::from_u64(0b1010000000000000000000);
        attacks[13] = BitBoard::from_u64(0b10100000000000000000000);
        attacks[14] = BitBoard::from_u64(0b101000000000000000000000);
        attacks[15] = BitBoard::from_u64(0b10000000000000000000000);

        // Black pawns
        attacks[48] = BitBoard::from_u64(0b100000000000000000000000000000000000000000);
        attacks[49] = BitBoard::from_u64(0b1010000000000000000000000000000000000000000);
        attacks[50] = BitBoard::from_u64(0b10100000000000000000000000000000000000000000);
        attacks[51] = BitBoard::from_u64(0b101000000000000000000000000000000000000000000);
        attacks[52] = BitBoard::from_u64(0b1010000000000000000000000000000000000000000000);
        attacks[53] = BitBoard::from_u64(0b10100000000000000000000000000000000000000000000);
        attacks[54] = BitBoard::from_u64(0b101000000000000000000000000000000000000000000000);
        attacks[55] = BitBoard::from_u64(0b10000000000000000000000000000000000000000000000);

        // Black knights
        attacks[57] = BitBoard::from_u64(0b1010000000000000000000000000000000000000000);
        attacks[62] = BitBoard::from_u64(0b101000000000000000000000000000000000000000000000);

        let attacked_by_white = BitBoard::from_u64(0b111111110000000000000000);
        let attacked_by_black = BitBoard::from_u64(0b111111110000000000000000000000000000000000000000);

        // let last_capture = None;
        // let castling_rights_history = Vec::new();
        // let halfmove_clock_history = Vec::new();

        let move_history = Vec::new();
        // let piece_giving_check = None;
        let check = false;
        let en_passant_square = None;

        Self {
            color_bitboards: bitboards,
            piece_bitboards: piece_boards,
            state: State::new(),
            attack_bitboards: attacks,
            attacked_by_white,
            attacked_by_black,
            /* was_last_move_capture: last_capture,
            castling_rights_history,
            halfmove_clock_history, */
            move_history,
            /* piece_giving_check, */
            check,
            en_passant_square,
        }    

    }

    // Prints out a visual representation of a given board state.
    pub fn print_position(&self) {
        let mut board = [[0; 8]; 8];
        for square in 0..64 {
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
    #[inline]
    pub fn piece_at(&self, square: u8) -> Option<(u8, Color)> {
        let mask: u64 = 1 << square;
        let color_mask = if self.color_bitboards[0].0 & mask != 0 {
            Color::White
        } else {
            Color::Black
        };

        let piece_index = (0..=5).find(|&i| self.piece_bitboards[i].0 & mask != 0).unwrap_or(6);

        if piece_index == 6 { return None; }

        Some((piece_index as u8, color_mask))
    }

    pub fn all_pieces(&self) -> BitBoard {
        self.color_bitboards[0] | self.color_bitboards[1]
    }

    pub fn piece_color(&self, square: u8) -> Color {
        if self.color_bitboards[0].contains(square) {
            Color::White
        } else if self.color_bitboards[1].contains(square) {
            Color::Black
        } else {
            panic!("No piece at square {} for position::piece_color()", square);
        }
    }

    pub fn make_move(&mut self, from: &u8, to: &u8) {
        let (piece, color) = self.piece_at(*from).unwrap();
        // Check for captures and update halfmove counter
        if self.piece_at(*to).is_some() {
            self.state.half_move_counter = 0;
            // Remove the captured piece from the color and piece bitboards
            let (captured_piece, captured_color) = self.piece_at(*to).unwrap();
            let captured_piece_index = match captured_piece {
                Piece::ROOK => 0,
                Piece::KNIGHT => 1,
                Piece::BISHOP => 2,
                Piece::QUEEN => 3,
                Piece::KING => 4,
                Piece::PAWN => 5,
                _ => panic!("Invalid piece"),
            };
            if captured_piece_index == 0 {
                match *to {
                    56 => self.state.castling_rights.0 &= !Castling::BLACK_QUEEN_SIDE,
                    63 => self.state.castling_rights.0 &= !Castling::BLACK_KING_SIDE,
                    0 => self.state.castling_rights.0 &= !Castling::WHITE_QUEEN_SIDE,
                    7 => self.state.castling_rights.0 &= !Castling::WHITE_KING_SIDE,
                    _ => (),
                }
            }
            // self.was_last_move_capture = Some(captured_piece);
            let to_mask = BitBoard::from_square(*to);
            self.color_bitboards[captured_color as usize] ^= to_mask;
            self.piece_bitboards[captured_piece_index] ^= to_mask;
        } else {
            self.state.half_move_counter += 1;
            // self.halfmove_clock_history.push(self.state.half_move_counter);
        }
        
        // If the move was not a capture, edit the flag accordingly
        // self.was_last_move_capture = None;
        
        // Update castling rights
        // self.castling_rights_history.push(self.state.castling_rights);
        match piece {
            Piece::KING => {
                match color {
                    Color::Black => self.state.castling_rights.0 &= !Castling::BLACK_CASTLING,
                    Color::White => self.state.castling_rights.0 &= !Castling::WHITE_CASTLING,
                }
                self.en_passant_square = None;
            },
            Piece::ROOK => {
                match color {
                    Color::Black => {
                        if *from == 56 {
                            self.state.castling_rights.0 &= !Castling::BLACK_QUEEN_SIDE;
                        } else if *from == 63 {
                            self.state.castling_rights.0 &= !Castling::BLACK_KING_SIDE;
                        }
                    },
                    Color::White => {
                        if *from == 0 {
                            self.state.castling_rights.0 &= !Castling::WHITE_QUEEN_SIDE;
                        } else if *from == 7 {
                            self.state.castling_rights.0 &= !Castling::WHITE_KING_SIDE;
                        }
                    },
                }
                self.en_passant_square = None;
            },
            Piece::PAWN => {
                self.state.half_move_counter = 0;
                // Set the en passant square if the pawn moved two squares
                if from / 8 == 1 && to / 8 == 3 {
                    self.en_passant_square = Some(*from + 8);
                } else if from / 8 == 6 && to / 8 == 4 {
                    self.en_passant_square = Some(*from - 8);
                } else {
                    self.en_passant_square = None;
                }
                // self.halfmove_clock_history.push(self.state.half_move_counter);
            }
            _ => self.en_passant_square = None,
        }

        let from_mask = BitBoard::from_square(*from);
        let to_mask = BitBoard::from_square(*to);
        let piece_index = match piece {
            Piece::ROOK => 0,
            Piece::KNIGHT => 1,
            Piece::BISHOP => 2,
            Piece::QUEEN => 3,
            Piece::KING => 4,
            Piece::PAWN => 5,
            _ => panic!("Invalid piece"),
        };
        self.color_bitboards[color as usize] ^= from_mask;
        self.color_bitboards[color as usize] |= to_mask;
        self.piece_bitboards[piece_index] ^= from_mask;
        self.piece_bitboards[piece_index] |= to_mask;

        self.state.switch_active_player();

        // Check for draw by 50 move rule
        if self.state.half_move_counter == 100 {
            self.state.game_result = GameResult(Results::DRAW);
        }

        self.move_history.push((*from, *to));

        // Assert that the color bitboards match the piece bitboards
        assert_eq!(self.color_bitboards[0] | self.color_bitboards[1],
            self.piece_bitboards[0] | self.piece_bitboards[1] | self.piece_bitboards[2] | self.piece_bitboards[3] | self.piece_bitboards[4] | self.piece_bitboards[5],
            "Inconsistent position initialization. Color bitboards do not match piece bitboards in move history {:?}.", self);
        // TODO: update en passant square
    }

    pub fn make_castling_move(&mut self, from: &u8, to: &u8) {
        // println!("Making castling move from {} to {}", from, to);
        let piece = self.piece_at(*from);
        match piece {
            Some((Piece::KING, _color)) => (),
            Some((Piece::ROOK, _color)) => (),
            _ => panic!("Trying to make castling move from {} to {} with move history {:?}",
                string_from_square(*from), string_from_square(*to), self.move_history),
        }
        let piece_type = piece.unwrap().0;
        let color = piece.unwrap().1;
        let piece_index = match piece_type {
            Piece::ROOK => 0,
            Piece::KING => 4,
            _ => panic!("Invalid piece"),
        };

        match to {
            // Check if the destination square for the rook is D1, D8, F1 or F8
            3 | 5 | 59 | 61  => {
                let rook_to_mask = BitBoard::from_square(*to);
                let rook_mask = BitBoard::from_square(*from);
                self.color_bitboards[color as usize] ^= rook_mask;
                self.color_bitboards[color as usize] |= rook_to_mask;
                self.piece_bitboards[piece_index] ^= rook_mask;
                self.piece_bitboards[piece_index] |= rook_to_mask;
            },
            _ => panic!("Invalid castling move"),
        }

        self.move_history.push((*from, *to));
        self.state.half_move_counter += 1;
        // self.halfmove_clock_history.push(self.state.half_move_counter);
        self.attack_bitboards[*from as usize] = BitBoard::empty();

    }

    /* pub fn unmake_move(&mut self, from: &Square, to: &Square) {
        // Find the piece that was moved
        let (piece, color) = self.piece_at(*to).unwrap();
        let piece_index = match piece {
            Piece::ROOK => 0,
            Piece::KNIGHT => 1,
            Piece::BISHOP => 2,
            Piece::QUEEN => 3,
            Piece::KING => 4,
            Piece::PAWN => 5,
            _ => panic!("Invalid piece"),
        };
        // Switch the active player back
        self.state.switch_active_player();

        // Move the piece back
        let from_mask = BitBoard::from_square(*from);
        let to_mask = BitBoard::from_square(*to);
        self.color_bitboards[color as usize] ^= to_mask;
        self.color_bitboards[color as usize] |= from_mask;
        self.piece_bitboards[piece_index] ^= to_mask;
        self.piece_bitboards[piece_index] |= from_mask;

        // Check if the move was a capture
        if let Some(captured_piece) = self.was_last_move_capture {
            let captured_piece_index = match captured_piece {
                Piece::ROOK => 0,
                Piece::KNIGHT => 1,
                Piece::BISHOP => 2,
                Piece::QUEEN => 3,
                Piece::KING => 4,
                Piece::PAWN => 5,
                _ => panic!("Invalid piece"),
            };
            let captured_mask = BitBoard::from_square(*to);
            self.color_bitboards[!self.state.active_player as usize] |= captured_mask;
            self.piece_bitboards[captured_piece_index] |= captured_mask;
        }

        // Revert castling rights by 1 index
        self.state.castling_rights = self.castling_rights_history.pop().unwrap();

        // Revert halfmove counter by 1 index
        self.state.half_move_counter = self.halfmove_clock_history.pop().unwrap();
    }
 */

    pub fn update_attack_maps(&mut self, attacker_square: u8, attacks: BitBoard) {
        self.attack_bitboards[attacker_square as usize] = attacks;
    }

    /* Returns true if the given square is under attack by the given color.
    / attack_bitboards contains a bitboard for every square that contains information which other squares the piece
    on that square attacks, if any. */ 
    #[inline]
    pub fn is_square_attacked_by_color(&self, square: u8, color: Color) -> bool {
        for i in 0..64 {
            // If the square is attacked by a piece on square i
            if self.attack_bitboards[i].contains(square) {
                // Check if the piece on square i is of the attacking color
                if self.color_bitboards[color as usize].contains(i as u8) {
                    return true;
                }
            }
        }
        false
    }

    // Only to be used for debugging purposes
    pub fn all_attacked_squares(&self, color: Color) -> BitBoard {
        let mut attacked_squares = BitBoard::empty();
        for i in 0..64 {
            if self.color_bitboards[color as usize].contains(i as u8) {
                attacked_squares |= self.attack_bitboards[i];
            }
        }
        attacked_squares
    }

    pub fn is_promotion(&self, start: &u8, end: &u8) -> bool {
        let (piece, color) = self.piece_at(*start).unwrap();
        if piece == Piece::PAWN {
            if color == Color::White && end / 8 == 7 || color == Color::Black && end / 8 == 0 {
                return true;
            }
        }
        false
    }

    pub fn promote_pawn(&mut self, square: u8, target_piece: u8) {
        let mask = BitBoard::from_square(square);
        let color = if self.color_bitboards[0].contains(square) {
            Color::White
        } else {
            Color::Black
        };
        // Get the piece index for the target piece
        let piece_index = match target_piece {
            Piece::ROOK => 0,
            Piece::KNIGHT => 1,
            Piece::BISHOP => 2,
            Piece::QUEEN => 3,
            _ => panic!("Invalid target piece"),
        };

        self.piece_bitboards[5] ^= mask;
        self.color_bitboards[color as usize] ^= mask;
        self.piece_bitboards[piece_index] |= mask;
        self.color_bitboards[color as usize] |= mask;
    }

    pub fn colorflip(&mut self) -> Position {
        let mut new_position = Position::new();
        // Flip the color bitboards
        new_position.color_bitboards[0] = self.color_bitboards[1].colorflip();
        new_position.color_bitboards[1] = self.color_bitboards[0].colorflip();
        // Flip the piece bitboards
        for i in 0..6 {
            new_position.piece_bitboards[i] = self.piece_bitboards[i].colorflip();
        }
        new_position.state.castling_rights = !self.state.castling_rights;
        new_position.state = self.state;
        new_position.state.switch_active_player();
        new_position
    }

    pub fn is_capture(&self, end: &u8) -> bool {
        if let Some(piece) = self.piece_at(*end) {
            if piece.1 != self.state.active_player {
                return true;
            }
        }
        false
    }

    pub fn piece_type_at(&self, square: u8) -> Option<u8> {
        if self.piece_bitboards[0].contains(square) {return Some(0)}
        if self.piece_bitboards[1].contains(square) {return Some(1)}
        if self.piece_bitboards[2].contains(square) {return Some(2)}
        if self.piece_bitboards[3].contains(square) {return Some(3)}
        if self.piece_bitboards[4].contains(square) {return Some(4)}
        if self.piece_bitboards[5].contains(square) {return Some(5)}
        None
    }
}