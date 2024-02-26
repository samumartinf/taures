# Taures
A small multiplatform chess game written in Rust using Tauri

TODO:
* Allow blocking pieces as a legal move
* Test legal move function
* Implement Hashing of the board to save previously checked positions

## Speed
As of (26/02/2024) move generations is awfully slow, sadly these are the numbers with pseudolegal moves. The following numbers are attached:
 Time taken for depth 1: 1.398959ms
 Time taken for depth 2: 15.015375ms
 Time taken for depth 3: 292.111292ms

