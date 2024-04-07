# AmselChess (Rust)

## Overview
This is a rust port of my unfinished amselChess engine, originally written in python.  
This workspace contains two library crates (precompute, types) and two binary crates (engine and precompute).  
You can specify the binary to run by using `cargo run --bin binary-name`.

## Features
* uses bitboards to represent boardstates to make evaluation ops cheaper
* uses precalculated magic bitboards to aid search efficiency

## ToDos
* refactor square Struct from using indices to an enum to allow for better performance when offsetting
* save precomputation outputs

## Known issues