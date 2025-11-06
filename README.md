# ♟️ Chess Engine in Rust

**Rust-based chess engine** built on top of the [`chess`](https://crates.io/crates/chess) crate.  
It uses the **Negamax algorithm** with **Alpha-Beta pruning** to compute optimal moves and a custom **static evaluation function** to assess chess positions.

---

## 🧠 Overview
This project aims to provide a clear and efficient foundation for building a chess AI — focusing on search efficiency, evaluation accuracy, and code simplicity.

---

## ✨ Features
- 🔍 **Negamax Algorithm with Alpha-Beta Pruning**  
  Efficiently finds the best move while pruning unnecessary branches.  
- 🧩 **Static Evaluation Function** — evaluates positions based on:  
  - Piece values  
  - Control of the center  
  - Piece activity  
  - Positional tables (reflecting chess theory and strategy)
- ⚡ **Simple Caching** — avoids redundant calculations.  
- ⏳ **Phase-Specific Evaluation** — adjusts logic for opening, middlegame, and endgame.

---

## ⚠️ Limitations
- ❌ No opening or endgame tablebases (memory and complexity constraints)  
- 🧱 Simplified evaluation — doesn’t yet recognize positional weaknesses  
- ⏱️ Search depth capped at around **6 plies**

---

## 🧩 Design Philosophy
This engine prioritizes:
- **Simplicity** — easy to understand and extend  
- **Efficiency** — uses core Rust features effectively  
- **Independence** — no reliance on external data sources  

While it doesn’t include advanced pruning, iterative deepening, or parallelism, it serves as a strong foundation for future upgrades.

---

## ⚙️ How It Works

### ♞ Move Generation
Uses the [`chess`](https://crates.io/crates/chess) crate for **fast, legal move generation**.

### 🧮 Position Evaluation
Combines:
- Piece values  
- Positional bonuses  
- Piece mobility  

Evaluations are always made from the perspective of the **side to move**.

### 🔁 Search Algorithm
Employs **Negamax with Alpha-Beta pruning** to explore game trees efficiently and identify the optimal move.

---

## 🚀 Getting Started

### 📦 Prerequisites
- **Rust** (install via [rust-lang.org](https://www.rust-lang.org/learn/get-started))

### 🧰 Installation
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
### Running the code:
```bash
cargo run
```
The program responds to the uci protocol. Type `.help` for the list of all commands.
## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE.txt) file for details.

## Author
Vilém Učík: [GitHub Profile](https://github.com/wiliams12)