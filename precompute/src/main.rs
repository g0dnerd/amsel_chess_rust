use precompute::rng::Rng;
use precompute::precompute_magics::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    
    // Tests the magics precomputation
    let mut rng = Rng::default();
    
    precompute_magics(&mut rng)?;
    Ok(())
}