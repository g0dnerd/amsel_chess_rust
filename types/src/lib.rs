use std::ops::Not;
pub mod bitboard;
pub mod position;
pub mod state;

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

pub struct Results;
impl Results {
    pub const ONGOING: u8 = 0b00010000;
    pub const DRAW: u8 = 0b00001000;
    pub const WHITE_VICTORY: u8 = 0b00000100;
    pub const BLACK_VICTORY: u8 = 0b00000010;
    pub const STALEMATE: u8 = 0b00000001;
}

// Provides the index for each piece type
#[derive(Debug)]
pub struct Piece;
impl Piece {
    pub const ROOK: u8 = 0;
    pub const KNIGHT: u8 = 1;
    pub const BISHOP: u8 = 2;
    pub const QUEEN: u8 = 3;
    pub const KING: u8 = 4;
    pub const PAWN: u8 = 5;
    pub const NO_PIECE: u8 = 6;
}

pub const PIECE_REPRESENTATIONS: [char; 13] = [
    '-', 'R', 'N', 'B', 'Q', 'K', 'P', 'r', 'n', 'b', 'q', 'k', 'p'
];

pub fn get_piece_representation(piece: u8) -> char {
    PIECE_REPRESENTATIONS[piece as usize]
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

pub mod types_utils {
    use crate::position::Position;

    pub fn try_square_offset(square: u8, dx: i8, dy: i8) -> Option<u8> {
        let (file, rank) = (square % 8, square / 8);
        let (new_file, new_rank) = (file as i8 + dx, rank as i8 + dy);
        if new_file < 0 || new_file > 7 || new_rank < 0 || new_rank > 7 {
            None
        } else {
            Some((new_rank * 8 + new_file) as u8)
        }
    }

    pub fn string_from_square(square: u8) -> String {
        let file = (square % 8) as u8 + 97;
        let rank = (square / 8) as u8 + 49;
        format!("{}{}", (file as char).to_ascii_uppercase(), (rank as char).to_ascii_uppercase())
    }

    pub fn fen_from_pos(pos: &Position) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                let square = rank * 8 + file;
                let piece: Option<(u8, crate::Color)> = pos.piece_at(square);
                match piece {
                    Some(piece) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        fen.push(crate::get_piece_representation(piece.0 + 6 * piece.1 as u8 + 1));
                    }
                    None => {
                        empty += 1;
                    }
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }
        fen.push(' ');
        fen.push(if pos.state.active_player == crate::Color::White { 'w' } else { 'b' });
        fen.push(' ');
        let mut castling = String::new();
        if pos.state.castling_rights.0 & crate::Castling::WHITE_KING_SIDE != 0 {
            castling.push('K');
        }
        if pos.state.castling_rights.0 & crate::Castling::WHITE_QUEEN_SIDE != 0 {
            castling.push('Q');
        }
        if pos.state.castling_rights.0 & crate::Castling::BLACK_KING_SIDE != 0 {
            castling.push('k');
        }
        if pos.state.castling_rights.0 & crate::Castling::BLACK_QUEEN_SIDE != 0 {
            castling.push('q');
        }
        if castling == "" {
            fen.push('-');
        } else {
            fen.push_str(&castling);
        }
        fen.push(' ');
        if let Some(en_passant) = pos.en_passant_square {
            fen.push_str(&crate::types_utils::string_from_square(en_passant));
        } else {
            fen.push('-');
        }
        fen.push(' ');
        fen.push_str(&pos.state.half_move_counter.to_string());
        fen.push(' ');
        fen.push_str(&pos.state.full_move_counter.to_string());
        fen
    }
}
    