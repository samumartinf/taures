# Taures - High-Performance Chess Engine

A modern, multiplatform chess game built with **Rust** and **Tauri**, featuring a blazing-fast chess engine that can evaluate 50,000+ positions per second.

## 🚀 Features

- **Lightning-fast chess engine** - Built in Rust with bitboard optimizations
- **Cross-platform** - Runs on Windows, macOS, and Linux using Tauri
- **Modern UI** - Built with Svelte 5 and Tailwind CSS
- **Opening book** - Includes popular opening moves for variety
- **Move validation** - Full chess rule compliance including castling, en passant, and promotion
- **Performance optimized** - 300x faster than the original implementation

## 🛠️ Tech Stack

### Backend (Rust)
- **Chess Engine**: Custom implementation with bitboard move generation
- **Performance**: Magic bitboard tables for sliding piece attacks
- **Memory Management**: Optimized for minimal allocations in search paths
- **Search Algorithm**: Alpha-beta pruning with move ordering

### Frontend (Svelte 5)
- **Framework**: Svelte 5 with runes (latest syntax)
- **Styling**: Tailwind CSS with custom UI components
- **Chessboard**: ChessboardJS integration
- **State Management**: Reactive state with Svelte 5 runes

### Desktop App
- **Framework**: Tauri v1 for native desktop performance
- **Build System**: Vite for fast development and optimized builds
- **Package Manager**: pnpm for dependency management

## 📦 Installation & Build

### Prerequisites
- **Node.js** 18+ and **pnpm** 9+
- **Rust** toolchain (rustup, cargo)
- **Tauri CLI**: `cargo install tauri-cli`

### Development Setup
```bash
# Clone the repository
git clone <your-repo-url>
cd taures

# Install dependencies
pnpm install

# Start development server
pnpm dev

# In another terminal, run Tauri dev
pnpm tauri dev
```

### Building for Production
```bash
# Build the application
pnpm tauri build

# The built app will be in src-tauri/target/release/
```

### Running Tests
```bash
# Run Rust tests
cargo test

# Run frontend tests
pnpm test

```

## 🏗️ Project Structure

```
taures/
├── src/                    # Svelte frontend
│   ├── lib/components/     # UI components
│   ├── routes/            # SvelteKit routes
│   └── app.html           # Main HTML template
├── src-tauri/             # Rust backend
│   ├── src/               # Chess engine source
│   │   ├── board.rs       # Board representation
│   │   ├── piece.rs       # Piece logic
│   │   ├── engine.rs      # Search engine
│   │   └── bitboard_movegen.rs # Bitboard optimizations
│   └── Cargo.toml         # Rust dependencies
├── static/                 # Static assets
└── package.json           # Frontend dependencies
```

## 🎯 Current Status

### ✅ Completed
- [x] Basic chess engine with array-based board representation
- [x] Full move generation and validation
- [x] Castling, en passant, and promotion support
- [x] **Migration to bitboards** - 300x performance improvement
- [x] **Magic bitboard tables** for sliding pieces
- [x] **Optimized search algorithm** with alpha-beta pruning
- [x] **Opening book** with popular moves
- [x] **Cross-platform desktop app** with Tauri


## 📊 Performance Metrics

| Metric | Before (Arrays) | After (Bitboards) | Improvement |
|--------|------------------|-------------------|-------------|
| Depth 2 Search | ~2-3 seconds | ~8ms | **300x faster** |
| Positions/Second | ~100-1,000 | 50,000+ | **50x faster** |
| Move Generation | Double work | Single pass | **2x reduction** |
| Legality Checking | Game cloning | Bitboard attacks | **100x faster** |

## 🤝 Contributing
I mean, you are more than welcome, but this was a project to learn Rust, so thread you might see some offensive code.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📝 License

This project is licensed under the Apache-2.0 OR MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Tauri team** for the excellent desktop framework
- **Svelte team** for the reactive frontend framework
- **Chess programming community** for bitboard optimization techniques

---

**Built with ❤️ using Rust, Tauri, and Svelte 5**
