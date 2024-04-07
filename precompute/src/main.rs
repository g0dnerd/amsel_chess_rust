use precompute::rng::Rng;
use precompute::precompute_magics::precompute_magics;
use precompute::populate_move_table::{make_table, write_magics, write_table};
use std::io::Result;

use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use precompute::magics::*;

fn main() -> Result<()> {
    
    // Tests the magics precomputation
    let mut rng = Rng::default();
    
    precompute_magics(&mut rng)?;
   
    let rook_table = precompute::populate_move_table::make_table(
        ROOK_TABLE_SIZE,
        &[(1, 0), (0, -1), (-1, 0), (0, 1)],
        ROOK_MAGICS,
    );
    let bishop_table = make_table(
        BISHOP_TABLE_SIZE,
        &[(1, 1), (1, -1), (-1, -1), (-1, 1)],
        BISHOP_MAGICS,
    );

    let mut out: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    out.push("magics.rs");
    let mut out = BufWriter::new(File::create(out).unwrap());

    write_magics("ROOK", ROOK_MAGICS, &mut out).unwrap();
    write_magics("BISHOP", BISHOP_MAGICS, &mut out).unwrap();
    write_table("ROOK", &rook_table, &mut out).unwrap();
    write_table("BISHOP", &bishop_table, &mut out).unwrap();

    Ok(())
}