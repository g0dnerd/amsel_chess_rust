# AmselChess (Rust)

## Overview
This is a rust port of my unfinished amselChess engine, originally written in python.  
This workspace contains two library crates (precompute, types) and two binary crates (engine and precompute).  
You can specify the binary to run by using `cargo run --bin binary-name`  
Compute and save magic bitboards by running `cargo build --bin precompute`

## Features
* uses bitboards to represent boardstates to make evaluation ops cheaper
* uses precalculated magic bitboards to aid search efficiency

## ToDos
* remove the second binary crate and move the precomputation work to `cargo build`for the main binary
* add tests
* finish move generation (needs to check legality - requires check detection)

## Known issues