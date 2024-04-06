use crate::bitboard::BitBoard;
use crate::{Color, Square, Pieces, get_piece_representation};
use crate::state::State;

/* A position contains the minimum amount of information necessary
/ for the engine to calculate moves and evaluate the board state. */ 

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    // Array of two BitBoards, one for each side
    color_bitboards: [BitBoard; 2],

    // Array of BitBoards, one for each piece type
    piece_boards: [BitBoard; 6],

    state: State,
}

impl Position {

    pub fn new () -> Position {
        let mut bitboards = [BitBoard::empty(); 2];
        let mut piece_boards = [BitBoard::empty(); 6];

        // Initialize the bitboard for both colors in their starting positions
        // White
        bitboards[0] = BitBoard::from_u64(0b1111111111111111);
        // Black
        bitboards[1] = BitBoard::from_u64(0b1111111111111111000000000000000000000000000000000000000000000000);

        // Initialize the piece bitboards for the respective starting positions
        // Rooks
        piece_boards[0] = BitBoard::from_u64(0b1000000100000000000000000000000000000000000000000000000010000001);
        // Knights
        piece_boards[1] = BitBoard::from_u64(0b100001000000000000000000000000000000000000000000000000001000010);
        // Bishops
        piece_boards[2] = BitBoard::from_u64(0b10010000000000000000000000000000000000000000000000000000100100);
        // Queens
        piece_boards[3] = BitBoard::from_u64(0b100000000000000000000000000000000000000000000000000000001000);
        // Kings
        piece_boards[4] = BitBoard::from_u64(0b1000000000000000000000000000000000000000000000000000000010000);
        // Pawns
        piece_boards[5] = BitBoard::from_u64(0b11111111000000000000000000000000000000001111111100000000);

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
        for row in board.iter().rev() {
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
        let color_mask = if self.color_bitboards[0].0 & mask != 0 {
            Color::White
        } else {
            Color::Black
        };
        
        let piece = if let Some(piece_index) = 
            (0..=5).find(|&i| self.piece_boards[i].0 & mask != 0) {
                
            match piece_index {
                0 => Pieces::ROOK,
                1 => Pieces::KNIGHT,
                2 => Pieces::BISHOP,
                3 => Pieces::QUEEN,
                4 => Pieces::KING,
                5 => Pieces::PAWN,
                _ => panic!("Invalid piece index"),
            }
        } else {
            return None;
        };
        
        Some((piece, color_mask))
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
            
            let square_bb = BitBoard::from_index(origin.0);
            let mut rook_moves = BitBoard::empty();
    
            // Calculate all possible rook moves relative to the current square
            rook_moves |= square_bb << 8;
            rook_moves |= square_bb >> 8;
            rook_moves |= square_bb.shift_east();
            rook_moves |= square_bb.shift_west();
    
            // Filter out moves that go off the board
            // let not_occupied = !(self.color_bitboards[0].0 | self.color_bitboards[1].0);
            rook_moves /*& not_occupied*/
    }

    pub fn get_knight_moves(&self, origin: &Square) -> BitBoard {
        
        let square_bb = BitBoard::from_index(origin.0);
        let mut knight_moves = BitBoard::empty();

        knight_moves |= square_bb.shift_east() << 16;
        knight_moves |= square_bb.shift_west() << 16;
        knight_moves |= square_bb.shift_east() >> 16;
        knight_moves |= square_bb.shift_west() >> 16;
        knight_moves |= square_bb.shift_west().shift_west() >> 8;
        knight_moves |= square_bb.shift_west().shift_west() << 8;
        knight_moves |= square_bb.shift_east().shift_east() >> 8;
        knight_moves |= square_bb.shift_east().shift_east() << 8;

        // Filter out moves that are blocked by occupied squares
        let not_occupied = !(self.color_bitboards[0] | self.color_bitboards[1]);
        knight_moves & not_occupied
    }

    pub fn get_bishop_moves(&self, origin: &Square) -> BitBoard {
    
        // Calculate bishop moves relative to the current square
        let square_bb = BitBoard::from_index(origin.0);
        let mut bishop_moves = BitBoard::empty();
    
        // Calculate all possible bishop moves relative to the current square
        bishop_moves |= square_bb.diagonal_north_east();
        bishop_moves |= square_bb.diagonal_north_west();
        bishop_moves |= square_bb.diagonal_south_east();
        bishop_moves |= square_bb.diagonal_south_west();
    
        // Filter out moves that go off the board
        let not_occupied = !(self.color_bitboards[0] | self.color_bitboards[1]);
        bishop_moves & not_occupied
    }

    pub fn get_queen_moves(&self, origin: &Square) -> BitBoard {
        let rook_moves = self.get_rook_moves(origin);
        let bishop_moves = self.get_bishop_moves(origin);
        let queen_moves = rook_moves | bishop_moves;
        queen_moves
    }

    // TODO: Pawn moves

}