use std::ops::Not;

pub mod bitboard;
pub mod position;
pub mod state;
pub mod square;

/* Represents a single square on the board.
/ Representation: 0-63, with 0 being a1 and 63 being h8. */

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}
impl From<bool> for Color {
    fn from(b: bool) -> Self {
        match b {
            false => Color::Black,
            true => Color::White,
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