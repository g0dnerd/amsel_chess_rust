use crate::{Color, Castling};
use crate::square::Square;

/* A state depicts additional information that is necessary to evaluate a position:
/ Castling rights, en passant square, halfmove clock and the active player. */

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    castling_rights: CastlingRights,
    en_passant_square: Option<Square>,
    half_move_counter: u8,
    active_player: Color,
}

impl State {
    pub fn new() -> Self {
        Self {
            castling_rights: CastlingRights::all(),
            en_passant_square: None,
            half_move_counter: 0,
            active_player: Color::White,
        }
    }
}

/* For performance reasons, we will use a single u8 to represent the castling rights:
/ The first 4 bits are unused,
/ bit 5 is white's king side castling rights,
/ bit 6 is white's queen side castling rights,
/ bit 7 is black's king side castling rights and
/ bit 8 is black's queen side castling rights. */
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    /* fn empty() -> Self {
        Self(Castling::NO_CASTLING)
    } */
    fn all() -> Self {
        Self::default()
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self(Castling::ANY_CASTLING)
    }
}