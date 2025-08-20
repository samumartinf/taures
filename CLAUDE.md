# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Taures is a multiplatform chess game built with Rust (Tauri backend) and SvelteKit (frontend). The project combines a chess engine written in Rust with a modern web-based UI using SvelteKit and TailwindCSS.

## Development Commands

### Frontend (SvelteKit)
- `pnpm dev` - Start development server
- `pnpm build` - Build for production  
- `pnpm preview` - Preview production build
- `pnpm check` - Run Svelte type checking
- `pnpm lint` - Run ESLint and Prettier checks
- `pnpm format` - Format code with Prettier
- `pnpm test` - Run all tests (integration + unit)
- `pnpm test:unit` - Run Vitest unit tests
- `pnpm test:integration` - Run Playwright integration tests

### Backend (Rust/Tauri)
- `cargo test` (in src-tauri/) - Run Rust tests
- `cargo bench` (in src-tauri/) - Run benchmarks
- `cargo tauri dev` - Start Tauri development mode
- `cargo tauri build` - Build Tauri application

## Architecture

### Chess Engine (Rust)
The core chess engine is implemented in Rust with the following key components:

- **Game (`src-tauri/src/lib.rs`)**: Main game state management, implements ChessGame trait for move validation, FEN parsing, and game logic
- **Board (`src-tauri/src/board.rs`)**: Chess board representation using both traditional array and bitboard formats. Handles castling rights, en passant, and board visualization
- **Piece (`src-tauri/src/piece.rs`)**: Individual piece logic with move generation for each piece type (Pawn, Rook, Knight, Bishop, Queen, King)
- **Engine (`src-tauri/src/lib.rs` engine module)**: Alpha-beta pruning search algorithm with position evaluation using piece-square tables (PSQT)
- **Position Helper**: Utilities for converting between algebraic notation (e.g., "e4") and internal board indices

### Frontend (SvelteKit)
- Built with SvelteKit using TypeScript
- Uses TailwindCSS for styling with shadcn/ui components  
- Integrates @chrisoakman/chessboardjs for chess board visualization
- Communicates with Rust backend via Tauri's invoke system

### Tauri Integration
The main.rs file exposes chess engine functions as Tauri commands:
- `play_move(source, target, promotion)` - Execute moves
- `get_legal_moves(source)` - Get legal moves for a piece
- `get_engine_move(depth)` - Get AI move at specified search depth
- `set_from_fen(fen)` - Set board position from FEN string
- `undo_move()`, `restart_game()` - Game controls

## Key Implementation Details

### Move Representation
Moves are represented as structs with source/target indices and optional promotion piece. The engine uses both pseudolegal move generation followed by legality filtering to handle check detection.

### Search Algorithm  
The engine implements alpha-beta pruning with position evaluation based on material values and piece-square tables. Search depth is configurable via the frontend.

### Board Representation
Dual representation using both 64-element u8 array for quick access and bitboards for future optimization. Pieces are encoded as bytes with flags for color, piece type, and position.

### Testing
Comprehensive test suite includes:
- Perft tests for move generation accuracy
- Individual piece movement validation  
- FEN serialization/deserialization
- Engine evaluation and search functionality
- Legal move filtering and check detection

## Build System
- Frontend uses Vite with SvelteKit
- Backend uses standard Cargo build system
- Tauri handles cross-platform builds and bundling
- Package manager is pnpm (version 9.4.0)

## Known TODOs
- Migration to bitboards for performance optimization
- Enhanced promotion handling beyond queen promotion
- Additional test coverage for castling and complex positions