use std::ops::Not;

mod bitboard;
pub mod position;
mod state;
pub mod rng;
pub mod precompute;

/* Represents a single square on the board.
/ Representation: 0-63, with 0 being a1 and 63 being h8. */
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Square(usize);

impl Square {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    /* Attempt to offset the square by a given delta.
    / If the new square is out of bounds OR on the edge of the board, return None. */
    pub fn offset(&self, file_offset: i8, rank_offset: i8) -> Option<Square> {
       let new_index = self.0 as i8 + file_offset + 8 * rank_offset;
       // Check if the new index overflows between ranks or files
        if self.0 < 8 && rank_offset == -1 {
        return None;
        }
        if self.0 >= 56 && rank_offset == 1 {
        return None;
        }
        if self.0 % 8 == 0 && file_offset == -1 {
            return None;
        }
        if self.0 % 8 == 7 && file_offset == 1 {
            return None;
        }
       if file_offset != 0 && rank_offset == 0 && new_index / 8 != self.0 as i8 / 8 {
        return None;
       }
       if  new_index < 0 || new_index > 63 {
           None
       } else {
        Some(Square(new_index as usize))
       }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Color {
    Black,
    White,
}
impl From<bool> for Color {
    fn from(b: bool) -> Self {
        match b {
            false => Color::White,
            true => Color::Black,
        }
    }
}
impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub enum GameResult {
    Ongoing,
    Checkmate,
    Stalemate,
    Draw,
}

// Provides the index for each piece type
pub struct Pieces;
impl Pieces {
    pub const ROOK: u8 = 0;
    pub const KNIGHT: u8 = 1;
    pub const BISHOP: u8 = 2;
    pub const QUEEN: u8 = 3;
    pub const KING: u8 = 4;
    pub const PAWN: u8 = 5;
}

pub const PIECE_REPRESENTATIONS: [char; 13] = [
    '-', 'R', 'N', 'B', 'Q', 'K', 'P', 'r', 'n', 'b', 'q', 'k', 'p'
];

pub fn get_piece_representation(piece: u8) -> char {
    PIECE_REPRESENTATIONS[piece as usize]
}

pub struct Castling;
impl Castling {
    pub const NO_CASTLING: u8 = 0;
    pub const WHITE_KING_SIDE: u8 = 0b00001000;
    pub const WHITE_QUEEN_SIDE: u8 = 0b00000100;
    pub const BLACK_KING_SIDE: u8 = 0b00000010;
    pub const BLACK_QUEEN_SIDE: u8 = 0b00000001;

    pub const BOTH_KING_SIDES: u8 = Self::WHITE_KING_SIDE | Self::BLACK_KING_SIDE;
    pub const BOTH_QUEEN_SIDES: u8 = Self::WHITE_QUEEN_SIDE | Self::BLACK_QUEEN_SIDE;
    pub const WHITE_CASTLING: u8 = Self::WHITE_KING_SIDE | Self::WHITE_QUEEN_SIDE;
    pub const BLACK_CASTLING: u8 = Self::BLACK_KING_SIDE | Self::BLACK_QUEEN_SIDE;
    pub const ANY_CASTLING: u8 = Self::WHITE_CASTLING | Self::BLACK_CASTLING;
}

pub fn get_printable_square_from_index(index: &usize) -> String {
    let file = index % 8;
    let rank = index / 8;
    let file_char = (file as u8 + 97) as char;
    let rank_char = (rank as u8 + 49) as char;
    format!("{}{}", file_char, rank_char)
}