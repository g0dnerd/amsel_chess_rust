use crate::rng::Rng;
use types::bitboard::BitBoard;
use types::square::Square;

/* This module contains the logic to precompute a magic number for a given slider piece and square
/ that perfectly maps input blockers into a hash table.
/ Heavy inspiration and support was taken from and many thanks are given to the magic-bitboards demo at
/ https://github.com/analog-hors/magic-bitboards-demo
/ licensed under the MIT License at https://spdx.org/licenses/MIT.html */

pub struct SlidingPiece {
    directions: [(i8, i8); 4],
}

impl SlidingPiece {
    // Returns a bitboard of all possible moves for a slider piece considering blockers.
    fn moves(&self, square: Square, blockers: BitBoard) -> BitBoard {
        let mut moves = BitBoard::empty();
        for &(dx, dy) in &self.directions {
            let mut ray = square;
            
            /* Find possible moves with the following procedure:
            /  1. Start at the piece's square.
            /  2. Try to offset the square by one of the four delta directions specified below.
            /  3. Loop terminates if that new square is in the list of blockers.
            /  4. If not, square gets added to legal moves. */
            while !blockers.contains(ray) {
                if let Some(offset_by_delta) = ray.offset(dx, dy) {
                    ray = offset_by_delta;
                    moves |= BitBoard::from_index(ray.0);
                } else {
                    break;
                }
            }
        }
        moves
    }

    /* Returns a bitboard of all potential squares that block the slider piece.
    / This later gets used as a mask to map the blocker squares to a hash table. */
    fn blocker_squares(&self, square: Square) -> BitBoard {
        let mut blockers = BitBoard::empty();

        /* Similar procedure as above:
        /  1. Check if the current square can be offset by the current delta.
        /  2. If not, the loop terminates, go to the next delta if there is one.
        /  3. If yes, add the square to the list of blocker squares.
        /  4. XOR square and the blocker BitBoard together. */
        for &(dx, dy) in &self.directions {
            let mut ray = square;
            while let Some(offset_by_delta) = ray.offset(dx, dy) {
                blockers |= BitBoard::from_index(ray.0);
                ray = offset_by_delta;
            }
        }
        blockers &= !BitBoard::from_index(square.0);
        blockers
    }
    
}

// Predefine directions for each slider piece.
pub const ROOK: SlidingPiece = SlidingPiece {
    directions: [(0, 1), (0, -1), (1, 0), (-1, 0)],
};

pub const BISHOP: SlidingPiece = SlidingPiece {
    directions: [(1, 1), (1, -1), (-1, 1), (-1, -1)],
};

struct MagicEntry {
    magic: u64,
    mask: BitBoard,
    shift: u8,
}

fn magic_index(entry: &MagicEntry, blockers: BitBoard) -> usize {
    // Apply the mask to the blockers to get the relevant squares.
    let blockers = blockers & entry.mask;

    // Multiply the blockers by the magic number to get the index.
    let hash = blockers.0.wrapping_mul(entry.magic);
    // println!("Attemping to find magic index for blockers: {:?} with hash {}", blockers, hash);

    // Shift the hash to the right by 64 - that entry's shift value to get the index.
    let index = (hash >> entry.shift) as usize;

    index
}

// Returns a magic number for a given slider piece and square.
fn compute_magic(
    sliding_piece: &SlidingPiece,
    square: Square,
    shift_amount: u8,
    rng_instance: &mut Rng
) -> (MagicEntry, Vec<BitBoard>) {

    // Get the applicable blocker squares from blocker_squares.
    let blockers = sliding_piece.blocker_squares(square);

    // Get the actual shift value.
    let shift = 64 - shift_amount;

    loop {

        /* Generate a random magic number with a low number of bits set by
        /  repeatedly generating random u64s and ANDing them together. */
        let magic = rng_instance.next_u64() & rng_instance.next_u64() & rng_instance.next_u64();

        // Create a new magic entry with the magic number, mask, and shift value.
        let magic_entry = MagicEntry {
            magic,
            mask: blockers,
            shift,
        };

        // If the result entry into the hash table is valid, return the magic number.
        if let Ok(magics) = attempt_magics(sliding_piece, square, &magic_entry) {
            return (magic_entry, magics);
        }
    }
}

struct TableFillError;

fn attempt_magics(
    sliding_piece: &SlidingPiece,
    square: Square,
    entry: &MagicEntry
) -> Result<Vec<BitBoard>, TableFillError> {

    // Get the actual index value.
    let shift = 64 - entry.shift;
    
    // Create a table with the corresponding size.
    let mut table = vec![BitBoard::empty(); 1 << shift];
    let mut blockers = BitBoard::empty();

    /* Iterate over all possible blocker combinations until a hash collision occurs or
    / all possible blocker combinations have been enumerated. */
    loop {
        let moves = sliding_piece.moves(square, blockers);
        let potential_entry = &mut table[magic_index(entry, blockers)];

        // Potential entry is still empty, assign the moves to it.
        if potential_entry.is_empty() {
            *potential_entry = moves;
        }
        // Potential entry is not empty, hash collision occurred. Return an error.
        else if *potential_entry != moves {
            return Err(TableFillError);
        }

        /* Enumerate all subsets of the current mask to get all relevant blocker squares.
        / This uses the Carry-Rippler method to traverse all possible subsets, see:
        / https://www.chessprogramming.org/Traversing_Subsets_of_a_Set */
        blockers.0 = blockers.0.wrapping_sub(entry.mask.0) & entry.mask.0;
        if blockers.is_empty() {
            // If all subsets have been enumerated, exit the loop.
            break;
        }
    }
    Ok(table)  
}

// Precompute and print magics for a given slider piece.
pub fn precompute_magics(
    sliding_piece: &SlidingPiece,
    piece_name: &str,
    rng_instance: &mut Rng) {
        println!("Computing magics for {}...", piece_name.to_lowercase());
        println!("With movement vectors: {:?}", sliding_piece.directions);
        println!(
            "pub const {}_MAGICS: &[MagicEntry; Square::NUM] = &[",
            piece_name.to_uppercase()
        );
        let mut table_length = 0;
        for square in 0..64 {
            let square = Square::new(square);
            let blockers_amount = sliding_piece.blocker_squares(square).count_ones() as u8;
            let (magic_entry, magics) = compute_magic(sliding_piece, square, blockers_amount, rng_instance);

            println!(
                "    MagicEntry {{ square: {}, mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, offset: {} }},",
                square.0, magic_entry.mask.0, magic_entry.magic, magic_entry.shift, table_length
            );
            table_length += magics.len();
        }
        println!("];");
        println!(
            "pub const {}_TABLE_SIZE: usize = {};",
            piece_name.to_uppercase(), table_length
        );
}
