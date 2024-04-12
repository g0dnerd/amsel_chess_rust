use crate::{Color, Castling, Results};
use crate::square::Square;

/* A state depicts additional information that is necessary to evaluate a position:
/ Castling rights, en passant square, halfmove clock and the active player. */

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub half_move_counter: u8,
    pub active_player: Color,
    pub game_result: GameResult,
}

impl State {
    pub fn new() -> Self {
        Self {
            castling_rights: CastlingRights::all(),
            en_passant_square: None,
            half_move_counter: 0,
            active_player: Color::White,
            game_result: GameResult::new(),
        }
    }

    pub fn switch_active_player(&mut self) {
        self.active_player = !self.active_player;
    }
}

/* Game result is stored in a u8:
/ The first 3 bits are unused,
/ bit 4 is ongoing,
/ bit 5 is a draw,
/ bit 6 is white's victory,
/ bit 7 is black's victory and
/ bit 8 is a stalemate.
*/
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct GameResult(pub u8);

impl GameResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ongoing(self) -> bool {
        self == GameResult(Results::ONGOING)
    }

}

impl Default for GameResult {
    fn default() -> Self {
        Self(Results::ONGOING)
    }
}

/* For performance reasons, we will use a single u8 to represent the castling rights:
/ The first 4 bits are unused,
/ bit 5 is white's king side castling rights,
/ bit 6 is white's queen side castling rights,
/ bit 7 is black's king side castling rights and
/ bit 8 is black's queen side castling rights. */
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    fn all() -> Self {
        Self::default()
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self(Castling::ANY_CASTLING)
    }
}