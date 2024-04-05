use bit::BitIndex;
use crate::{Color, Pieces};

/* A piece is stored as an 8-bit number:
/ first bit: color (0 for white, 1 for black)
/ next six bits: piece type (order: rook, knight, bishop, queen, king, pawn)
/ last bit: whether the piece has moved (0 for no, 1 for yes)
*/

#[derive(Debug, Default)]
pub struct Piece(pub u8);

impl Piece {
    /* Returns the color for the pieces. Uses the bit method from the BitIndex trait to get the first bit
    / (with 7 being the LSB) and converts
    / the resulting bool to a color by using the into method */
    pub fn get_color(&self) -> Color {
        self.0.bit(7).into()
    }

    pub fn has_moved(&self) -> bool {
        self.0.bit(0)
    }

    /* Returns the piece type using the Pieces struct.
    / Uses the bit method from the BitIndex trait to check the piece type bits (see above) */ 
    pub fn get_piece_type(&self) -> u8 {
        // Assert that the piece is not invalid
        debug_assert!(!self.is_invalid());
        if self.0.bit(6) {
            Pieces::PAWN
        } else if self.0.bit(5) {
            Pieces::KING
        } else if self.0.bit(4) {
            Pieces::QUEEN
        } else if self.0.bit(3) {
            Pieces::BISHOP
        } else if self.0.bit(2) {
            Pieces::KNIGHT
        } else if self.0.bit(1) {
            Pieces::ROOK
        } else {
            panic!("Invalid piece type")
        }
    }

    // Debug method to check if the piece is invalid
    pub fn is_invalid(&self) -> bool {
        // Return true if bits 1-6 are all 0
        self.0 & 0b01111110 == 0
    }
}

