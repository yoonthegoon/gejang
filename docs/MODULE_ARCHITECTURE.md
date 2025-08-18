# Module Architecture

Gejang is structured around core chess concepts, with modules organized to handle different aspects of chess position representation, error handling, and game state management. The architecture follows Rust's module system and emphasizes separation of concerns.

## Module Structure

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

## Core Components

**Bitboard Module**: Handles efficient 64-bit representation of chess board squares using bitwise operations. The `Bitboard` struct provides methods for bit manipulation, directional shifting, and square operations. The `Square` struct represents individual chess squares with file/rank coordinate system and supports algebraic notation parsing.

**Position Module**: Represents complete chess game states using FEN (Forsyth-Edwards Notation) standard. The `Position` struct orchestrates all position components including piece placement, active color, castling rights, en passant targets, and move counters. Supporting structs handle specific aspects like `ActiveColor` for turn tracking, `CastlingAvailability` for castling rights, and `PiecePlacementData` for piece locations using bitboards.

**Error Handling**: Centralized error management through the `Error` enum with comprehensive validation and graceful handling of parsing failures.

## Design Principles

- **Separation of Concerns**: Each module has a single, well-defined responsibility
- **Efficient Representation**: Uses bitboards for fast bitwise operations optimized for chess engine performance
- **Standards Compliance**: Full FEN support and UCI protocol compatibility
- **Extensibility**: Modular design allows easy addition of features like move generation, position evaluation, and search algorithms