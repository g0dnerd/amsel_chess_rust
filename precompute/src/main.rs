use precompute::rng::Rng;
use precompute::precompute_magics::*;

fn main() {
    
    // Tests the magics precomputation
    let mut rng = Rng::default();
    precompute_magics(&ROOK, "Rook", &mut rng);
    precompute_magics(&BISHOP, "Bishop", &mut rng);
}