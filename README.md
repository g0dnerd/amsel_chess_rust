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
* add castling to king's move generation
* add en passant
* add player checkmate detection (for now, only engine can be checkmated because user doesn't have every legal move generated)
* add promotion (auto-promote to queens for now)

### Done
* dynamic storage of slider paths & attacks to further increase performance
* cleaned up basic game loop & move generation
* move game logic for the move loop out of main.rs
* fixed illegal move removal while in check

## Known issues