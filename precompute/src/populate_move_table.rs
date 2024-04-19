use std::io::prelude::*;
use types::{
    bitboard::BitBoard,
    types_utils::*,
};
use crate::magics::*;


fn magic_index(entry: &MagicTableEntry, blockers: BitBoard) -> usize {
    let blockers = blockers.0 & entry.mask;
    let hash = blockers.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

fn slider_moves(slider_deltas: &[(i8, i8)], square: u8, blockers: BitBoard) -> BitBoard {
    let mut moves = BitBoard::empty();
    for &(df, dr) in slider_deltas {
        let mut ray = square;
        while !blockers.contains(ray) {
            if let Some(shifted) = try_square_offset(ray, df, dr) {
                ray = shifted;
                moves |= BitBoard::from_square(ray);
            } else {
                break;
            }
        }
    }
    moves
}

pub fn make_table(
    table_size: usize,
    slider_deltas: &[(i8, i8)],
    magics: &[MagicTableEntry; 64],
) -> Vec<BitBoard> {
    let mut table = vec![BitBoard::empty(); table_size];
    for square in 0..64 {
        let magic_entry = &magics[square];
        let mask = BitBoard(magic_entry.mask);

        let mut blockers = BitBoard::empty();
        loop {
            let moves = slider_moves(slider_deltas, square as u8, blockers);
            table[magic_index(magic_entry, blockers)] = moves;

            // Carry-Rippler trick that enumerates all subsets of the mask, getting us all blockers.
            // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set#All_Subsets_of_any_Set
            blockers.0 = blockers.0.wrapping_sub(mask.0) & mask.0;
            if blockers.is_empty() {
                break;
            }
        }
    }
    table
}

pub fn write_table(name: &str, table: &[BitBoard], out: &mut impl Write) -> std::io::Result<()> {
    write!(out, "pub const {}_MOVES: &[u64; {}] = &[", name, table.len())?;
    for entry in table {
        write!(out, "{},", entry.0)?;
    }
    write!(out, "];")?;
    Ok(())
}

pub fn write_magics(
    name: &str,
    magics: &[MagicTableEntry; 64],
    out: &mut impl Write,
) -> std::io::Result<()> {
    write!(
        out,
        "pub const {}_MAGICS: &[MagicEntry; Square::NUM] = &[",
        name
    )?;
    for entry in magics {
        write!(
            out,
            "MagicEntry {{ mask: {}, magic: {}, shift: {}, offset: {} }},",
            entry.mask, entry.magic, entry.shift, entry.offset
        )?;
    }
    write!(out, "];")?;
    Ok(())
}