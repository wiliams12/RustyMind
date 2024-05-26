use std::{process::ChildStdout, str::FromStr};

use super::search::Engine;
use chess::{Board,ChessMove, Rank, File, Piece, Square};


pub struct Game {
    pub board: Board,
    ai: Engine,
    depth: Option<i32>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::default(),
            ai: Engine::new(),
            depth: None,
        }
    }

    pub fn set_depth(&mut self, depth: i32) {
        self.depth = Some(depth);
    }

    pub fn play(&mut self) -> ChessMove{
        let depth = if self.depth == None {
            2
        }
        else {
            self.depth.unwrap()
        };
        let best_move = self.ai.play(&self.board, depth);
        if best_move == None {
            panic!("Internal error, Invalid position")
        }
        best_move.unwrap()
    }

    pub fn set_board(&mut self, fen: &str, moves: Vec<&str>) {
        // Initialize the board from FEN and create a new game
        let mut board = Board::from_str(fen).unwrap();

        for mv in moves {
            let source = Square::make_square(
                Rank::from_index(mv[1..2].parse::<usize>().unwrap() - 1),
                File::from_index((mv.as_bytes()[0] - b'a') as usize),
            );
            let dest = Square::make_square(
                Rank::from_index(mv[3..4].parse::<usize>().unwrap() - 1),
                File::from_index((mv.as_bytes()[2] - b'a') as usize),
            );
            let promotion = if mv.len() > 4 {
                Some(match mv.as_bytes()[4] {
                    b'q' => Piece::Queen,
                    b'r' => Piece::Rook,
                    b'b' => Piece::Bishop,
                    b'n' => Piece::Knight,
                    _ => unreachable!(),
                })
            } else {
                None
            };

            let chess_move = ChessMove::new(source, dest, promotion);

            // Apply the move to the board
            board = board.make_move_new(chess_move);
        }

        // Update the board to reflect the final state
        self.board = board;
    }
}
