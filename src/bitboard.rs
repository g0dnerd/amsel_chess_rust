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

    pub fn from_index(index: usize) -> Self {
        assert!(index < 64, "Index out of bounds");
        Self(1 << index)
    }

    pub fn shift_north(&self) -> Self {
        Self(self.0 << 8)
    }

    pub fn shift_south(&self) -> Self {
        Self(self.0 >> 8)
    }

    pub fn shift_east(&self) -> Self {
        Self(self.0 << 1 & 0xFEFE_FEFE_FEFE_FEFE)
    }

    pub fn shift_west(&self) -> Self {
        Self(self.0 >> 1 & 0x7F7F_7F7F_7F7F_7F)
    }

    // Shifts the bits diagonally to the north-east
    pub fn diagonal_north_east(&self) -> Self {
        Self((self.0 & 0xFEFE_FEFE_FEFE_FEFE) << 9)
    }

    // Shifts the bits diagonally to the north-west
    pub fn diagonal_north_west(&self) -> Self {
        Self((self.0 & 0x7F7F_7F7F_7F7F_7F7F) << 7)
    }

    // Shifts the bits diagonally to the south-east
    pub fn diagonal_south_east(&self) -> Self {
        Self((self.0 & 0xFEFE_FEFE_FEFE_FEFE) >> 7)
    }

    // Shifts the bits diagonally to the south-west
    pub fn diagonal_south_west(&self) -> Self {
        Self((self.0 & 0x7F7F_7F7F_7F7F_7F7F) >> 9)
    }

    // Returns whether the BitBoard is empty
    /* pub fn is_empty(&self) -> bool {
        self.0 == 0
    } */

} 