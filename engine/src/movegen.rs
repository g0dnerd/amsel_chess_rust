use types::bitboard::BitBoard;
// use types::square::Square;
use precompute::magics::*;

fn magic_index(entry: &MagicTableEntry, blockers: BitBoard) -> usize {
    let blockers = blockers.0 & entry.mask;
    let hash = blockers.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

/* pub fn get_rook_moves(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_entry = &ROOK_MAGICS[square as usize];
    let index = magic_index(magic_entry, blockers);
    BitBoard(precompute::magics::ROOK_MOVES[index])
} */