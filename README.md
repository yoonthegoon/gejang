<!-- markdownlint-disable MD033 MD041 -->
## ⚠️ Status ⚠️

Gejang is incomplete and not yet ready for use. \
There are still many features that make a chess engine missing. \
The [usage](#usage) section only currently descirbes how Gejang is planned to be used.

---

<div align="center">

![Gejang Logo][gejang-logo]

### Gejang

A free UCI chess engine.

[![GitHub Release][github-release]][latest-release]
[![GitHub Actions][github-actions-workflow-status]][github-actions]

</div>

## Installation

### Requirements

- [Cargo][cargo]

### Installation Instructions

```shell
cargo install --git https://github.com/yoonthegoon/gejang.git
```

## Usage

Gejang is normally used with a GUI that supports the UCI protocol.
Refer to your GUI's documentation on how to use a UCI engine.
Use `which gejang` to find the path to the executable.

Gejang can also be used in the terminal by running `gejang` and entering UCI commands.

```shell
$ gejang
> uci
id name Gejang
id author Yunis Yilmaz
uciok

> isready
readyok

> position startpos
> go depth 10
...
bestmove e2e4

> position startpos moves e2e4 e7e5
> go movetime 1000
...
bestmove g1f3

> quit
```

For more information on the UCI protocol, the specification can be found [here][uci-specification].

## Support

If you find something isn't working or doesn't seem right, or want to suggest a new feature,
please [open an issue][new-issue].
If you simply have a question, please [open a discussion][new-discussion].

## Architecture

Gejang is structured around core chess concepts, with modules organized to handle different aspects of chess position representation, error handling, and game state management. The architecture follows Rust's module system and emphasizes separation of concerns.

### Module Structure

```
src/
├── main.rs              # Entry point
├── error.rs             # Error handling
├── result.rs            # Result type aliases
├── bitboard/            # Bitboard operations and chess squares
│   ├── mod.rs
│   ├── bitboard.rs
│   └── square.rs
└── position/            # Chess position representation
    ├── mod.rs
    ├── position.rs
    ├── active_color.rs
    ├── castling_availability.rs
    └── piece_placement_data.rs
```

### Core Components

**Bitboard Module**: Handles efficient 64-bit representation of chess board squares using bitwise operations. The `Bitboard` struct provides methods for bit manipulation, directional shifting, and square operations. The `Square` struct represents individual chess squares with file/rank coordinate system and supports algebraic notation parsing.

**Position Module**: Represents complete chess game states using FEN (Forsyth-Edwards Notation) standard. The `Position` struct orchestrates all position components including piece placement, active color, castling rights, en passant targets, and move counters. Supporting structs handle specific aspects like `ActiveColor` for turn tracking, `CastlingAvailability` for castling rights, and `PiecePlacementData` for piece locations using bitboards.

**Error Handling**: Centralized error management through the `Error` enum with comprehensive validation and graceful handling of parsing failures.

### Design Principles

- **Separation of Concerns**: Each module has a single, well-defined responsibility
- **Efficient Representation**: Uses bitboards for fast bitwise operations optimized for chess engine performance
- **Standards Compliance**: Full FEN support and UCI protocol compatibility
- **Extensibility**: Modular design allows easy addition of features like move generation, position evaluation, and search algorithms

## Contributing

This project is currently in its early stages and not yet open to contributions.
Once the project is more mature, contributions will be welcome.

## Acknowledgment

- [Chess Programming Wiki][chess-programming-wiki]
- [Sebastian Lague's Chess Coding Adventure][chess-coding-adventure]
- [Stockfish][stockfish]
- [Chess Engine Testers Discord Server][chess-engine-testers]

## License

[![GNU GPLv3][gpl-v3-logo]][license]

<!-- markdownlint-enable MD033 MD041 -->

[cargo]:                          https://doc.rust-lang.org/cargo/getting-started/installation.html
[chess-coding-adventure]:         https://github.com/SebLague/Chess-Coding-Adventure
[chess-engine-testers]:           https://discord.gg/EN25UBJ8C5
[chess-programming-wiki]:         https://www.chessprogramming.org/Main_Page
[gejang-logo]:                    /assets/logo.svg
[github-actions]:                 https://github.com/yoonthegoon/gejang/actions
[github-actions-workflow-status]: https://img.shields.io/github/actions/workflow/status/yoonthegoon/gejang/rust.yml
[github-release]:                 https://img.shields.io/github/v/release/yoonthegoon/gejang?include_prereleases
[gpl-v3-logo]:                    /assets/gpl-v3-logo.svg
[latest-release]:                 https://github.com/yoonthegoon/gejang/releases/latest
[license]:                        LICENSE
[new-discussion]:                 https://github.com/yoonthegoon/gejang/discussions/new/choose
[new-issue]:                      https://github.com/yoonthegoon/gejang/issues/new/choose
[stockfish]:                      https://github.com/official-stockfish/Stockfish
[uci-specification]:              engine-interface_2.txt
