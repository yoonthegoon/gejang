# Gejang Module Architecture

This document explains the modular structure of Gejang, a UCI chess engine written in Rust. Each module has a specific responsibility and together they form the foundation for chess position representation and manipulation.

## Overview

Gejang is structured around core chess concepts, with modules organized to handle different aspects of chess position representation, error handling, and game state management. The architecture follows Rust's module system and emphasizes separation of concerns.

## Module Hierarchy

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

## Core Modules

### 1. Main Module (`main.rs`)

**Purpose**: Application entry point and module coordination.

**Functionality**:
- Imports all necessary modules (`bitboard`, `error`, `position`, `result`)
- Creates a new chess position using `Position::new()`
- Demonstrates basic position creation and display

**Key Interactions**:
- Uses `Position` from the position module to create a starting chess position
- Acts as the coordination point for all other modules

### 2. Error Handling (`error.rs`)

**Purpose**: Centralized error management for the entire application.

**Key Components**:
- `Error` enum with `ParseError(String)` variant
- Implements `From<ParseIntError>` for seamless error conversion
- Provides a consistent error handling interface across modules

**Interactions**:
- Used by all parsing operations in `Square`, `Position`, `ActiveColor`, etc.
- Consumed by the `result.rs` module to create the application's Result type

### 3. Result Type (`result.rs`)

**Purpose**: Provides a convenient type alias for the application's Result type.

**Functionality**:
- Defines `Result<T>` as `std::result::Result<T, Error>`
- Simplifies error handling throughout the codebase
- Used extensively in parsing and validation operations

## Bitboard Module (`bitboard/`)

The bitboard module handles low-level chess board representation using bitwise operations.

### Bitboard (`bitboard/bitboard.rs`)

**Purpose**: Efficient 64-bit representation of chess board squares using bitwise operations.

**Key Features**:
- Uses `u64` to represent all 64 squares of a chess board
- Provides constants for files (A-H) and ranks (1-8)
- Supports directional shifting for piece movement calculations
- Implements bit manipulation operations (ls1b, ms1b, pop_count)

**Key Methods**:
- `from_square()`: Creates a bitboard with a single square set
- `shift()`: Moves bits in eight directions (N, S, E, W, NE, NW, SE, SW)
- `ls1b()`: Finds the least significant 1 bit (lowest set square)
- `ms1b()`: Finds the most significant 1 bit (highest set square)
- `pop_count()`: Counts the number of set bits
- `xor_square()`: Toggles a specific square on/off

**Interactions**:
- Works closely with `Square` to convert between square representations
- Used extensively by `PiecePlacementData` to store piece positions
- Provides the foundation for efficient chess position manipulation

### Square (`bitboard/square.rs`)

**Purpose**: Represents individual chess squares with file/rank coordinate system.

**Key Features**:
- Stores square as a single `u8` value (0-63)
- Provides conversion between different square representations
- Implements parsing from algebraic notation (e.g., "a1", "h8")
- Supports Display trait for human-readable output

**Key Methods**:
- `new(rank, file)`: Creates square from rank/file coordinates
- `from_u8()`: Creates square from 0-63 index
- `file()` and `rank()`: Extract file and rank components
- String parsing and display for algebraic notation

**Interactions**:
- Used by `Bitboard` for square-specific operations
- Used throughout position module for square references
- Provides the bridge between human-readable notation and internal representation

### Direction Enum (`bitboard/mod.rs`)

**Purpose**: Defines eight compass directions for bitboard shifting operations.

**Directions**: North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest

**Interactions**:
- Used by `Bitboard::shift()` method for directional movement
- Essential for calculating piece attacks and movement patterns

## Position Module (`position/`)

The position module represents complete chess game states using the FEN (Forsyth-Edwards Notation) standard.

### Position (`position/position.rs`)

**Purpose**: Central data structure representing a complete chess position.

**Key Components**:
- `piece_placement_data`: Where all pieces are located
- `active_color`: Whose turn it is to move
- `castling_availability`: Which castling rights remain
- `en_passant_target_square`: En passant capture square (if any)
- `halfmove_clock`: Moves since last pawn move or capture
- `fullmove_number`: Full moves in the game

**Key Methods**:
- `new()`: Creates standard starting position
- `from_fen()`: Parses position from FEN string
- Implements `Display` and `FromStr` for FEN conversion

**Interactions**:
- Orchestrates all position sub-components
- Uses `PiecePlacementData`, `ActiveColor`, `CastlingAvailability`
- Depends on `Square` for en passant target representation
- Used by main application as the primary game state

### Active Color (`position/active_color.rs`)

**Purpose**: Tracks whose turn it is to move.

**Values**:
- `White`: White to move
- `Black`: Black to move

**FEN Representation**: "w" for White, "b" for Black

**Interactions**:
- Used by `Position` to track game turn
- Simple enum with parsing and display capabilities

### Castling Availability (`position/castling_availability.rs`)

**Purpose**: Tracks remaining castling rights for both sides.

**Fields**:
- `white_king_side`: Can White castle kingside?
- `white_queen_side`: Can White castle queenside?
- `black_king_side`: Can Black castle kingside?
- `black_queen_side`: Can Black castle queenside?

**FEN Representation**: "KQkq" format (uppercase=White, lowercase=Black)

**Interactions**:
- Used by `Position` to maintain castling state
- Critical for legal move generation (not yet implemented)

### Piece Placement Data (`position/piece_placement_data.rs`)

**Purpose**: Manages the locations of all pieces on the chess board.

**Internal Structure**:
- Array of 12 `Bitboard` objects (one per piece type)
- Index mapping: [K, Q, R, B, N, P, k, q, r, b, n, p]
- Each bitboard tracks all pieces of that specific type

**Key Features**:
- Converts to/from FEN piece placement notation
- Uses bitboards for efficient piece location storage
- Supports piece lookup by square

**Interactions**:
- Heavily uses `Bitboard` for piece storage
- Uses `Square` for coordinate system
- Central to `Position` for representing board state
- Provides foundation for move generation and game logic

## Module Interactions Flow

```
┌─────────────┐
│   main.rs   │ ←── Entry point
└──────┬──────┘
       │ uses Position::new()
       ▼
┌─────────────────────┐
│   position/         │
│   ├── position.rs   │ ←── Orchestrates chess state
│   ├── active_color  │ ←── Turn tracking
│   ├── castling_...  │ ←── Castling rights
│   └── piece_place.. │ ←── Piece locations
└──────┬──────────────┘
       │ uses Bitboard & Square
       ▼
┌─────────────────────┐
│   bitboard/         │
│   ├── bitboard.rs   │ ←── Efficient board ops
│   ├── square.rs     │ ←── Square representation
│   └── mod.rs        │ ←── Direction enum
└─────────────────────┘
       ▲
       │ error handling
┌─────────────────────┐
│   error.rs          │ ←── Error definitions
│   result.rs         │ ←── Result type alias
└─────────────────────┘
```

## Data Flow

1. **Initialization**: `main.rs` creates a new `Position` using the standard starting position FEN
2. **FEN Parsing**: `Position::from_str()` parses FEN string into components
3. **Component Creation**: Each FEN component creates its respective data structure
4. **Bitboard Population**: `PiecePlacementData` uses `Bitboard::xor_square()` to set piece positions
5. **Square Conversion**: `Square` objects provide coordinate system for piece placement
6. **Display**: Complete position converts back to FEN format for output

## Design Principles

### 1. **Separation of Concerns**
- Each module has a single, well-defined responsibility
- Bitboards handle low-level board representation
- Position components handle high-level game state

### 2. **Efficient Representation**
- Uses bitboards for fast bitwise operations
- 64-bit integers represent entire board states
- Optimized for chess engine performance requirements

### 3. **Standards Compliance**
- Full FEN (Forsyth-Edwards Notation) support
- Proper chess coordinate systems (files a-h, ranks 1-8)
- Compatible with UCI chess engine protocol

### 4. **Error Handling**
- Centralized error management through `Error` enum
- Comprehensive validation of input data
- Graceful handling of parsing failures

### 5. **Extensibility**
- Modular design allows easy addition of new features
- Bitboard foundation supports complex chess algorithms
- Clean interfaces between modules for future expansion

## Future Extensions

The current architecture provides a solid foundation for implementing:

- **Move Generation**: Using bitboard operations for legal move calculation
- **Position Evaluation**: Scoring positions for engine decision-making
- **Search Algorithms**: Minimax, alpha-beta pruning, and advanced search techniques
- **UCI Protocol**: Communication with chess GUIs
- **Opening Books**: Integration of opening move databases
- **Endgame Tablebases**: Access to perfect endgame knowledge

This modular architecture ensures that each new feature can be added without disrupting existing functionality, maintaining clean separation of concerns throughout the chess engine's evolution.