# Chess Program with Rust

## Overview
This project is a Rust chess engine built on top of the `chess` crate. The program's primary goal is to compute optimal moves using the negamax algorithm with alpha-beta pruning and to implement a robust static evaluation function for assessing chess positions.

## Features
- **Negamax Algorithm with Alpha-Beta Pruning**: Ensures efficient and optimal move selection.
- **Static Evaluation Function**: Evaluates positions based on:
  - Piece values.
  - Control of the center.
  - Piece activity.
  - Positional tables reflecting chess theory and strategy.
- **Simple Caching**: Speeds up calculations by avoiding redundant computations.
- **Phase-Specific Evaluation**: Adjusts evaluations based on game phases (opening, middlegame, endgame).

## Limitations
- Does not include opening or endgame tablebases (due to memory and complexity constraints).
- Evaluation function is fairly simple and does not recognize positional weaknesses.
- The depth of the search is limited (6 plies is still manageable).

## Design Decisions
The program was designed with simplicity, efficiency, and independence from external data sources in mind. While it does not employ advanced pruning techniques, iterative deepening, or parallelism, it serves as a solid base for future development.

## How It Works
### Move Generation
Leverages the Rust `chess` crate for fast and accurate legal move generation.

### Position Evaluation
- Combines piece values, positional bonuses, and piece mobility.
- Evaluates positions from the perspective of the side to move.

### Search Algorithm
Applies negamax with alpha-beta pruning to search for optimal moves efficiently.

## Getting Started

### Prerequisites
- **Rust**: Ensure you have Rust installed. [Download Rust here](https://www.rust-lang.org/learn/get-started).

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/wiliams12/RustyMind
    ```
2. Navigate to the project directory:
    ```bash
    cd RustyMind
    ```
### Running the Program
Compiling the code:
```bash
cargo build
```
Running the code:
```bash
cargo run
```
The program responds to the uci protocol. Type `.help` for the list of all commands.
## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Author
Williams12: [GitHub Profile](https://github.com/wiliams12)