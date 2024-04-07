use types::bitboard::BitBoard;
use types::square::Square;
use types::position::Position;
use crate::magics::*;
use precompute::magics::MagicTableEntry;
use precompute::precompute_magics::{ROOK, BISHOP};

/* Use our ray-scanning algorithm from the precompute module to get potential blockers for a piece,
/ then AND the result with the all_pieces BitBoard to get the actual blockers. */
pub fn get_actual_blockers(directions: &[(i8, i8)], square: Square, position: &Position) -> BitBoard {
    let mut blockers = BitBoard::empty();
    // XOR the square and the blocker BitBoard together to remove the piece we are analyzing from the list of potential blockers.
    let all_pieces = position.all_pieces() ^ BitBoard::from_square(square);
    for &(dx, dy) in directions {
        let mut ray = square;
        while let Some(offset_by_delta) = ray.try_offset(dx, dy) {
            blockers |= BitBoard::from_square(ray);
            ray = offset_by_delta;
        }
    }
    blockers &= !BitBoard::from_square(square);
    blockers & all_pieces
}

fn magic_index(entry: &MagicTableEntry, blockers: BitBoard) -> usize {
    let blockers = blockers.0 & entry.mask;
    let hash = blockers.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

pub fn get_rook_moves_from_position(square: Square, position: &Position) -> BitBoard {
    let blockers = get_actual_blockers(&ROOK.directions, square, position);
    let magic_entry = &ROOK_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    let mut moves = BitBoard::from_u64(ROOK_MOVES[index]);

    // Remove all moves that would capture a piece of the same color
    let color = position.piece_at(square).unwrap().1;
    moves = moves & position.color_bitboards[color as usize];
    moves
}

pub fn get_rook_moves_from_blockers(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_entry = &ROOK_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    BitBoard(ROOK_MOVES[index])
}

pub fn get_bishop_moves_from_position(square: Square, position: &Position) -> BitBoard {
    let blockers = get_actual_blockers(&BISHOP.directions, square, position);
    let magic_entry = &BISHOP_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    let mut moves = BitBoard::from_u64(BISHOP_MOVES[index]);

    // Remove all moves that would capture a piece of the same color
    let color = position.piece_at(square).unwrap().1;
    moves = moves & position.color_bitboards[color as usize];
    moves
}

pub fn get_bishop_moves_from_blockers(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_entry = &BISHOP_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    BitBoard(BISHOP_MOVES[index])
}

pub fn get_queen_moves_from_position(square: Square, position: &Position) -> BitBoard {
    get_rook_moves_from_position(square, position) | get_bishop_moves_from_position(square, position)
}

pub fn get_queen_moves_from_blockers(square: Square, blockers: BitBoard) -> BitBoard {
    get_rook_moves_from_blockers(square, blockers) | get_bishop_moves_from_blockers(square, blockers)
}