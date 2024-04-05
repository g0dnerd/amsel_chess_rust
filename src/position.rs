use crate::bitboard::BitBoard;
use crate::{Color, Square, Pieces, get_piece_representation};
use crate::state::State;

/* A position contains the minimum amount of information necessary
/ for the engine to calculate moves and evaluate the board state. */ 

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    // Array of two BitBoards, one for each side
    color_bitboards: [BitBoard; 2],

    // Array of arrays of BitBoards, one for each piece type for each side
    piece_boards: [[BitBoard; 6]; 2],

    state: State,
}

impl Position {

    pub fn new () -> Position {
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

        Self {
            color_bitboards: bitboards,
            piece_boards,
            state: State::new(),
        }    

    }

    // Prints out a visual representation of a given board state.
    pub fn print_position(&self) {
        let mut board = [[0; 8]; 8];
        for i in 0..64 {
            match self.piece_at(&Square(i)) {
                Some((piece, color)) => {
                    let x = i % 8;
                    let y = i / 8;
                    board[y][x] = match color {
                        Color::White => piece + 1,
                        Color::Black => piece + 7,
                    };
                },
                None => ()
            }
        }
        for row in board.iter() {
            for square in row.iter() {
                print!("{} ", get_piece_representation(*square as u8));
            }
            println!();
        }
    }

    // Returns the piece at a given square or None if the square is empty
    pub fn piece_at(&self, square: &Square) -> Option<(u8, Color)> {
        let index = square.0;
        let mask: u64 = 1 << index;
        if self.piece_boards[0][0].0 & mask != 0 {
            Some((Pieces::ROOK, Color::White))
        } 
        else if self.piece_boards[0][1].0 & mask != 0 {
            Some((Pieces::KNIGHT, Color::White))
        }
        else if self.piece_boards[0][2].0 & mask != 0 {
            Some((Pieces::BISHOP, Color::White))
        }
        else if self.piece_boards[0][3].0 & mask != 0 {
            Some((Pieces::QUEEN, Color::White))
        }
        else if self.piece_boards[0][4].0 & mask != 0 {
            Some((Pieces::KING, Color::White))
        }
        else if self.piece_boards[0][5].0 & mask != 0 {
            Some((Pieces::PAWN, Color::White))
        }
        else if self.piece_boards[1][0].0 & mask != 0 {
            Some((Pieces::ROOK, Color::Black))
        }
        else if self.piece_boards[1][1].0 & mask != 0 {
            Some((Pieces::KNIGHT, Color::Black))
        }
        else if self.piece_boards[1][2].0 & mask != 0 {
            Some((Pieces::BISHOP, Color::Black))
        }
        else if self.piece_boards[1][3].0 & mask != 0 {
            Some((Pieces::QUEEN, Color::Black))
        }
        else if self.piece_boards[1][4].0 & mask != 0 {
            Some((Pieces::KING, Color::Black))
        }
        else if self.piece_boards[1][5].0 & mask != 0 {
            Some((Pieces::PAWN, Color::Black))
        } else {
            None
        }
    }

    pub fn get_legal_moves_by_square(&self, _from: Square) -> BitBoard {
        match self.piece_at(&_from) {
            Some((Pieces::ROOK, Color::White)) => self.get_rook_moves(&_from),
            Some((Pieces::ROOK, Color::Black)) => self.get_rook_moves(&_from),
            Some((Pieces::KNIGHT, Color::White)) => self.get_knight_moves(&_from),
            Some((Pieces::KNIGHT, Color::Black)) => self.get_knight_moves(&_from),
            Some((Pieces::BISHOP, Color::White)) => self.get_bishop_moves(&_from),
            Some((Pieces::BISHOP, Color::Black)) => self.get_bishop_moves(&_from),
            Some((Pieces::QUEEN, Color::White)) => self.get_queen_moves(&_from),
            Some((Pieces::QUEEN, Color::Black)) => self.get_queen_moves(&_from),
            /* Some((Pieces::KING, Color::White)) => self.get_king_moves(_from),
            Some((Pieces::KING, Color::Black)) => self.get_king_moves(_from),
            Some((Pieces::PAWN, Color::White)) => self.get_pawn_moves(_from),
            Some((Pieces::PAWN, Color::Black)) => self.get_pawn_moves(_from), */
            _ => BitBoard::empty(),
        }
    }

    /* The get_xyz_moves methods use bit masks and bitwise operations to check
    / for legal moves for each piece type. This considers blocked squares and paths
    / as well as the board boundaries.
    / TODOs:
        - consider captures
        - consider pins (should that be done here or in the move generation?)
     */

    pub fn get_rook_moves(&self, origin: &Square) -> BitBoard {
        let mut moves: u64 = 0;

        // Generate horizontal moves
        let file = origin.0 % 8;
        let rank = origin.0 / 8;
        let mut mask = BitBoard::from_u64(0x0101010101010101).0 << file;
        mask &= !BitBoard::from_u64(1).0 << origin.0;
        moves |= mask << (rank * 8);

        // Generate vertical moves
        mask = BitBoard::from_u64(0xFF).0 << (rank * 8);
        mask &= !BitBoard::from_u64(1).0 << origin.0;
        moves |= mask.rotate_right(file as u32);

        // Apply blocking masks
        let not_occupied = !(self.color_bitboards[0].0 | self.color_bitboards[1].0);
        let blocking_mask = !not_occupied;

        moves &= blocking_mask;

        BitBoard::from_u64(moves)
    }

    pub fn get_knight_moves(&self, origin: &Square) -> BitBoard {
        
        let square_bb = BitBoard::from_index(origin.0);
        let mut knight_moves: u64 = 0;

        knight_moves |= square_bb.shift_north().shift_north().shift_east().0;
        knight_moves |= square_bb.shift_north().shift_north().shift_west().0;
        knight_moves |= square_bb.shift_south().shift_south().shift_east().0;
        knight_moves |= square_bb.shift_south().shift_south().shift_west().0;
        knight_moves |= square_bb.shift_west().shift_west().shift_south().0;
        knight_moves |= square_bb.shift_west().shift_west().shift_north().0;
        knight_moves |= square_bb.shift_east().shift_east().shift_south().0;
        knight_moves |= square_bb.shift_east().shift_east().shift_north().0;

        // Filter out moves that are blocked by occupied squares
        let not_occupied = !(self.color_bitboards[0].0 | self.color_bitboards[1].0);
        BitBoard::from_u64(knight_moves & not_occupied)
    }

    pub fn get_bishop_moves(&self, origin: &Square) -> BitBoard {
    
        // Calculate bishop moves relative to the current square
        let square_bb = BitBoard::from_index(origin.0);
        let mut bishop_moves: u64 = 0;
    
        // Calculate all possible bishop moves relative to the current square
        bishop_moves |= square_bb.diagonal_north_east().0;
        bishop_moves |= square_bb.diagonal_north_west().0;
        bishop_moves |= square_bb.diagonal_south_east().0;
        bishop_moves |= square_bb.diagonal_south_west().0;
    
        // Filter out moves that go off the board
        let not_occupied = !(self.color_bitboards[0].0 | self.color_bitboards[1].0);
        BitBoard::from_u64(bishop_moves & not_occupied)
    }

    pub fn get_queen_moves(&self, origin: &Square) -> BitBoard {
        let rook_moves = self.get_rook_moves(origin);
        let bishop_moves = self.get_bishop_moves(origin);
        let queen_moves = rook_moves.0 | bishop_moves.0;
        BitBoard::from_u64(queen_moves)
    }

    // TODO: Pawn moves

}