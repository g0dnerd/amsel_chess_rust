use crate::position::Position;
use crate::{Color, GameResult};

pub struct Game {
    position: Position,
    current_player: Color,
    game_result: GameResult
}

impl Game {
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            current_player: Color::White,
            game_result: GameResult::Ongoing,
        }
    }

    pub fn make_move(&mut self, _from: u8, _to: u8) {
        todo!()
    }
}