# AmselChess (Rust)

## Overview
This is a rust port of my unfinished amselChess engine, originally written in python.  
This workspace contains two library crates (precompute, types) and two binary crates (engine and precompute).  
You can specify the binary to run by using `cargo run --bin binary-name`  
Compute and save magic bitboards by running `cargo build --bin precompute`

## Features
* uses bitboards to represent boardstates to make evaluation ops cheaper
* uses precalculated magic bitboards to aid search efficiency
* dynamic attacker storage to reduce computation time each time legal moves are generated

## ToDos
* remove the second binary crate and move the precomputation work to `cargo build` for the main binary
* add en passant
* refactor square class to a simple index
* add lower/upper bound flags to transposition table entries

### Done
* dynamic storage of slider paths & attacks to further increase performance
* cleaned up basic game loop & move generation
* move game logic for the move loop out of main.rs
* fixed illegal move removal while in check
* switch to rayon for multithreading
* add transposition tables to search module

## Known issues
* decoupled attack map synchronization causes issues (surprise!)
* castling sometimes not legal when it should be, 99% sure it's related to point above
* the higher the depth, the less likely AI is to not hang a queen? still confused by whether it optimizes for the correct player