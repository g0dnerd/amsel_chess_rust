// Module for the BitBoard struct
// A BitBoard is a 64-bit integer that represents the state of a chess board.
// They are implemented here with bit 0 being a1 and bit 63 being h8.

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    // Returns an empty BitBoard
    pub fn empty() -> Self {
        Self(0)
    }

    // Returns a BitBoard from the given u64
    pub fn from_u64(u: u64) -> Self {
        Self(u)
    }

    // Returns whether the BitBoard is empty
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }


} 