use crate::bitboard::BitBoard;
use crate::{Color, Square, Pieces, get_piece_representation};

// A position contains the minimum amount of information necessary to calculate moves and evaluate the board state.

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    // Array of two BitBoards, one for each side
    color_bitboards: [BitBoard; 2],

    // Array of arrays of BitBoards, one for each piece type for each side
    piece_boards: [[BitBoard; 6]; 2],
}

/* Returns the piece at a given square
/ or None if the square is empty */
pub fn piece_at(pos: &Position, square: Square) -> Option<(u8, Color)> {
    let index = square.0;
    println!("Checking for a piece at index {}", index);
    let mask: u64 = 1 << index;
    if pos.piece_boards[0][0].0 & mask != 0 {
        Some((Pieces::ROOK, Color::White))
    } 
    else if pos.piece_boards[0][1].0 & mask != 0 {
        Some((Pieces::KNIGHT, Color::White))
    }
    else if pos.piece_boards[0][2].0 & mask != 0 {
        Some((Pieces::BISHOP, Color::White))
    }
    else if pos.piece_boards[0][3].0 & mask != 0 {
        Some((Pieces::QUEEN, Color::White))
    }
    else if pos.piece_boards[0][4].0 & mask != 0 {
        Some((Pieces::KING, Color::White))
    }
    else if pos.piece_boards[0][5].0 & mask != 0 {
        Some((Pieces::PAWN, Color::White))
    }
    else if pos.piece_boards[1][0].0 & mask != 0 {
        Some((Pieces::ROOK, Color::Black))
    }
    else if pos.piece_boards[1][1].0 & mask != 0 {
        Some((Pieces::KNIGHT, Color::Black))
    }
    else if pos.piece_boards[1][2].0 & mask != 0 {
        Some((Pieces::BISHOP, Color::Black))
    }
    else if pos.piece_boards[1][3].0 & mask != 0 {
        Some((Pieces::QUEEN, Color::Black))
    }
    else if pos.piece_boards[1][4].0 & mask != 0 {
        Some((Pieces::KING, Color::Black))
    }
    else if pos.piece_boards[1][5].0 & mask != 0 {
        Some((Pieces::PAWN, Color::Black))
    } else {
        println!("No piece found at square {}", index);
        None
    }
}

pub fn init_bitboards() -> Position {
    let mut bitboards = [BitBoard::empty(); 2];
    let mut piece_boards = [[BitBoard::empty(); 6]; 2];

    // Initialize the bitboard for both colors in their starting positions
    // White
    bitboards[0] = BitBoard::from_u64(0b1111111111111111);
    // Black
    bitboards[1] = BitBoard::from_u64(0b1111111111111111000000000000000000000000000000000000000000000000);

    // Initialize the piece bitboards for the white pieces in their starting positions
    // White Rooks
    piece_boards[0][0] = BitBoard::from_u64(0b10000001);
    // White Knights
    piece_boards[0][1] = BitBoard::from_u64(0b1000010);
    // White Bishops
    piece_boards[0][2] = BitBoard::from_u64(0b100100);
    // White Queens
    piece_boards[0][3] = BitBoard::from_u64(0b1000);
    // White King
    piece_boards[0][4] = BitBoard::from_u64(0b10000);
    // White Pawns
    piece_boards[0][5] = BitBoard::from_u64(0b1111111100000000);

    // Initialize the piece bitboards for the black pieces in their starting positions
    // Black Rooks
    piece_boards[1][0] = BitBoard::from_u64(0b1000000100000000000000000000000000000000000000000000000000000000);
    // Black Knights
    piece_boards[1][1] = BitBoard::from_u64(0b100001000000000000000000000000000000000000000000000000000000000);
    // Black Bishops
    piece_boards[1][2] = BitBoard::from_u64(0b10010000000000000000000000000000000000000000000000000000000000);
    // Black Queens
    piece_boards[1][3] = BitBoard::from_u64(0b100000000000000000000000000000000000000000000000000000000000);
    // Black King
    piece_boards[1][4] = BitBoard::from_u64(0b1000000000000000000000000000000000000000000000000000000000000);
    // Black Pawns
    piece_boards[1][5] = BitBoard::from_u64(0b11111111000000000000000000000000000000000000000000000000);

    Position {
        color_bitboards: bitboards,
        piece_boards,
    }    

}

// Prints out a visual representation of a given board state.
pub fn print_position(pos: Position) {
    let mut board = [[0; 8]; 8];
    for i in 0..64 {
        println!("Checking square {}", i);
        match piece_at(&pos, Square(i)) {
            Some((piece, color)) => {
                println!("Found a piece at square {}: {:?}", i, color);
                let x = i % 8;
                let y = i / 8;
                board[y][x] = match color {
                    Color::White => piece + 1,
                    Color::Black => piece + 7,
                };
            },
            None => println!("No piece found at square {}", i),
        }
    }
    for row in board.iter() {
        for square in row.iter() {
            print!("{} ", get_piece_representation(*square as u8));
        }
        println!();
    }
}