use crate::rng::Rng;
use types::{
    bitboard::BitBoard,
    types_utils::*,
};
use std::{
    io::{
        self,
        prelude::*,
        Error,
    },
    fs::File,
};

/* This module contains the logic to precompute a magic number for a given slider piece and square
/ that perfectly maps input blockers into a hash table.
/ Heavy inspiration and support was taken from and many thanks are given to the magic-bitboards demo at
/ https://github.com/analog-hors/magic-bitboards-demo
/ licensed under the MIT License at https://spdx.org/licenses/MIT.html */

#[derive(PartialEq)]
pub struct SlidingPiece {
    pub directions: [(i8, i8); 4],
}

impl SlidingPiece {
    // Returns a bitboard of all possible moves for a slider piece considering blockers.
    fn moves(&self, square: u8, blockers: BitBoard) -> BitBoard {
        let mut moves = BitBoard::empty();
        for &(dx, dy) in &self.directions {
            let mut ray = square;
            
            /* Find possible moves with the following procedure:
            /  1. Start at the piece's square.
            /  2. Try to offset the square by one of the four delta directions specified below.
            /  3. Loop terminates if that new square is in the list of blockers.
            /  4. If not, square gets added to legal moves. */
            while !blockers.contains(ray) {
                if let Some(offset_by_delta) = try_square_offset(ray, dx, dy) {
                    ray = offset_by_delta;
                    moves |= BitBoard::from_square(ray);
                } else {
                    break;
                }
            }
        }
        moves
    }

    /* Returns a bitboard of all potential squares that block the slider piece.
    / This later gets used as a mask to map the blocker squares to a hash table. */
    fn blocker_squares(&self, square: u8) -> BitBoard {
        let mut blockers = BitBoard::empty();

        /* Similar procedure as above:
        /  1. Check if the current square can be offset by the current delta.
        /  2. If not, the loop terminates, go to the next delta if there is one.
        /  3. If yes, add the square to the list of blocker squares.
        /  4. XOR square and the blocker BitBoard together. */
        for &(dx, dy) in &self.directions {
            let mut ray = square;
            while let Some(offset_by_delta) = try_square_offset(ray, dx, dy) {
                blockers |= BitBoard::from_square(ray);
                ray = offset_by_delta;
            }
        }
        blockers &= !BitBoard::from_square(square);
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
    square: u8,
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
    square: u8,
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

// Precompute and save magics for all slider piece.
pub fn precompute_magics(
    rng_instance: &mut Rng) -> Result<(), Error> {
        let path = "./precompute/src/magics.rs";
        println!("Precomputing magics in path {}", path);
        std::fs::remove_file(path).ok();
        let mut output_file = File::create(path)?;
        let line = "pub struct MagicTableEntry {
            pub mask: u64,
            pub magic: u64,
            pub shift: u8,
            pub offset: u32,
        }\n";
        write!(output_file, "{}\n", line)?;
        for sliding_piece in &[ROOK, BISHOP] {
            let piece_name = if sliding_piece == &ROOK { "rook" } else { "bishop" };
            println!("\nComputing magics for {}", piece_name);
            let line = format!(
                "pub const {}_MAGICS: &[MagicEntry; 64] = &[",
                piece_name.to_uppercase()
            );
            write!(output_file, "{}\n", line)?;
            let mut table_length = 0;
            for square in 0..64 {
                let blockers_amount = sliding_piece.blocker_squares(square).count_ones() as u8;
                let (magic_entry, magics) = compute_magic(sliding_piece, square, blockers_amount, rng_instance);

                let line = format!(
                    "    MagicEntry {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, offset: {} }},",
                    magic_entry.mask.0, magic_entry.magic, magic_entry.shift, table_length
                );
                write!(output_file, "{}\n", line)?;
                print!("\rEntry {} of 64 written to file.", square as usize + 1);
                io::stdout().flush().unwrap();
                table_length += magics.len();
            }

            let line = format!("];");
            write!(output_file, "{}\n", line)?;
            let line = format!(
                "pub const {}_TABLE_SIZE: usize = {};",
                piece_name.to_uppercase(), table_length
            );
            write!(output_file, "{}\n\n", line)?;
        }
        Ok(())
}