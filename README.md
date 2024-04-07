# AmselChess (Rust)

## Overview
This is a rust port of my unfinished amselChess engine, originally written in python.

## Features
* uses bitboards to represent boardstates to make evaluation ops cheaper

## ToDos
* refactor square Struct from using indices to an enum to allow for better performance when offsetting
* refactor the precomputation to a separate module
* save precomputation outputs

## Done
* implementation of a magics finder and magic bitboards
* maybe reduce the total number of bitboards again (1 for each color, 1 for each piece type, can & them together)

## Known issues