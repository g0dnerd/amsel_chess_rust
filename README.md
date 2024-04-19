# AmselChess (Rust)

## Overview
This is a rust port of and the spiritual successor to my then unfinished amselChess engine, originally written in python.  
This workspace contains two library crates (precompute, types) and two binary crates (engine and precompute).  
You can specify the binary to run by using `cargo run --bin binary-name`  
Compute and save magic bitboards by running `cargo build --bin precompute`

## Features
* uses bitboards to represent boardstates to make evaluation ops cheaper
* uses precalculated magic bitboards to aid search efficiency
* dynamic attacker storage to reduce computation time each time legal moves are generated

## ToDos
* add en passant
* add lower/upper bound flags to transposition table entries
* remove the second binary crate and move the precomputation work to `cargo build` for the main binary

### Done
* dynamic storage of slider paths & attacks to further increase performance
* cleaned up basic game loop & move generation
* move game logic for the move loop out of main.rs
* refactored Square module to simple u8s
* fixed illegal move removal while in check
* switch to rayon for multithreading
* add transposition tables to search module

## Known issues