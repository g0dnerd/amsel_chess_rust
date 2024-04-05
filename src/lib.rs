use std::ops::Not;

mod bitboard;
pub mod position;
mod piece;
// mod game;

/* Represents a single square on the board.
/ Representation: 0-63, with 0 being a1 and 63 being h8. */
#[derive(Debug)]
pub struct Square(usize);

#[derive(Debug, Clone, Copy)]
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
    '-', 'r', 'n', 'b', 'q', 'k', 'p', 'R', 'N', 'B', 'Q', 'K', 'P'
];

pub fn get_piece_representation(piece: u8) -> char {
    PIECE_REPRESENTATIONS[piece as usize]
}