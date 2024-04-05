use crate::bitboard::BitBoard;

// A position contains the minimum amount of information necessary to calculate moves and evaluate the board state.

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    // Array of two BitBoards, one for each side
    bitboards: [BitBoard; 2],

    // Array of arrays of BitBoards, one for each piece type for each side
    piece_boards: [[BitBoard; 6]; 2],

    // Necessary information on the current state of the game (castling rights, en passant square, etc.)
    // state: State,
}

impl Position {
    // Returns an empty position
    #[allow(dead_code)]
    pub fn new() -> Self {
        // Initialize the bitboards and piece_boards arrays with empty BitBoards
        let bitboards = [BitBoard::new(); 2];
        let piece_boards = [[BitBoard::new(); 6]; 2];
        // let state = State::new();
        Position {
            bitboards,
            piece_boards,
            // state,
        }
    }

    // Prints out a visual representation of the current board state.
    #[allow(dead_code)]
    pub fn print(&self) {
        // Print the board state
        for _rank in (0..8).rev() {
            for _file in 0..8 {
                todo!();
            }
            println!();
        }
    }

    /* Returns the piece at a given square
    / TODO: Null handling - should this return an Option?
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        todo!()
    } */
}