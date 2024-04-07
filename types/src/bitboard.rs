use std::ops::*;

use crate::square::Square;

// Module for the BitBoard struct
// A BitBoard is a 64-bit integer that represents the state of a chess board.
// They are implemented here with bit 0 being a1 and bit 63 being h8.

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);

/* Use macros to implement mathematical operations for the BitBoard struct.
/ This macro code is taken from the wonderful magic-bitboards demo at
/ https://github.com/analog-hors/magic-bitboards-demo
/ licensed under the MIT License at https://spdx.org/licenses/MIT.html */
macro_rules! impl_math_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait for BitBoard {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                Self($trait::$fn(self.0, other.0))
            }
        }
    )*};
}
impl_math_ops! {
    BitAnd, bitand;
    BitOr, bitor;
    BitXor, bitxor;
}

macro_rules! impl_math_assign_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait for BitBoard {
            fn $fn(&mut self, other: Self) {
                $trait::$fn(&mut self.0, other.0)
            }
        }
    )*};
}
impl_math_assign_ops! {
    BitAndAssign, bitand_assign;
    BitOrAssign, bitor_assign;
    BitXorAssign, bitxor_assign;
}

macro_rules! impl_shift_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait<usize> for BitBoard {
            type Output = Self;

            fn $fn(self, other: usize) -> Self::Output {
                Self(self.0.$fn(other as u32))
            }
        }
    )*};
}

impl_shift_ops! {
    Shl, shl;
    Shr, shr;
}


impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitBoard {
    // Returns an empty BitBoard
    pub fn empty() -> Self {
        Self(0)
    }

    // Returns a BitBoard from the given u64
    pub fn from_u64(u: u64) -> Self {
        Self(u)
    }

    pub fn from_square(square: Square) -> Self {
        Self(1 << square as usize)
    }

    pub fn from_index(index: usize) -> Self {
        assert!(index < 64, "Index out of bounds");
        Self(1 << index)
    }

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, square: Square) -> bool {
        !(self & BitBoard::from_index(square as usize)).is_empty()
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

    pub fn squares_from_bb(&self) -> Vec<Square> {
        let mut squares = Vec::new();
        let mut bb = self.0;
        while bb != 0 {
            let square = Square::index(bb.trailing_zeros() as usize);
            squares.push(square);
            bb &= bb - 1;
        }
        squares
    }

} 