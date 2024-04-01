# Taures
A small multiplatform chess game written in Rust using Tauri

## Building
To build the project you need to have Rust installed. You can install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install). After installing Rust you can build the project by running the following command in the root of the project:
```bash
pnpm tauri dev # For development
pnpm tauri build # For building the project
```

TODO:
- [x] Allow blocking pieces as a legal move
- [x ] Test legal move function
- [x] Implement Hashing of the board to save previously checked positions
- [ ] Speed up move generation

## Speed
As of (26/02/2024) move generations is awfully slow, sadly these are the numbers with pseudolegal moves. The following numbers are attached:
 Time taken for depth 1: 1.398959ms
 Time taken for depth 2: 15.015375ms
 Time taken for depth 3: 292.111292ms

