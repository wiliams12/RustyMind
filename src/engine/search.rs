use super::evaluation::*;
use super::helpers::*;

use chess::{Board, BoardStatus, CacheTable, ChessMove, Color, MoveGen};
use rand::prelude::*;
use rand::thread_rng;

pub struct Engine {
    // Engine structure
    random: ThreadRng,
    cache: CacheTable<i32>,
}

impl Engine {
    pub fn new() -> Engine {
        // Object generator
        Engine {
            random: thread_rng(),
            cache: CacheTable::new(268_435_456, 0),
        }
    }
    pub fn play(&mut self, board: &Board, depth: i32) -> Option<ChessMove> {
        // Calls the necessary function to return the best move
        self.negamax_root(board, depth)
    }

    fn negamax_root(&mut self, board: &Board, depth: i32) -> Option<ChessMove> {
        // Root function to the negamax
        // Returns the best move selected from recursive calls to the negamax function
        match board.status() {
            BoardStatus::Checkmate => {
                println!("checkmate");
                return None;
            }
            BoardStatus::Stalemate => {
                println!("stalemate");
                return None;
            }
            BoardStatus::Ongoing => (),
        }

        let mut best_move: Option<ChessMove> = None;
        let mut max_eval = -100000;
        for mv in reorder_moves(&board, MoveGen::new_legal(&board)) {
            let mut eval = -self.negamax(&board.make_move_new(mv), -99999, 99999, depth - 1);
            // Adds a little random cushion to the moves evaluation so the selection is randomised between similarly evaluated moves
            let random = self.random.gen_range(-1..=1);
            eval += random;
            if eval > max_eval {
                max_eval = eval;
                best_move = Some(mv);
            }
        }
        best_move
    }

    fn negamax(&mut self, board: &Board, mut alpha: i32, beta: i32, depth: i32) -> i32 {
        // The <= will make the program to act as depth=1 when depth<1
        if depth <= 0
            || match board.status() {
                BoardStatus::Checkmate => true,
                BoardStatus::Stalemate => true,
                BoardStatus::Ongoing => false,
            }
        {
            return self.quiescence_search(board, alpha, beta, 0);
        }

        for mv in reorder_moves(&board, MoveGen::new_legal(&board)) {
            let eval = -self.negamax(&board.make_move_new(mv), -beta, -alpha, depth - 1);
            if eval > alpha {
                alpha = eval;
            }

            if alpha >= beta {
                break;
            }
        }
        alpha
    }

    fn quiescence_search(&mut self, board: &Board, mut alpha: i32, beta: i32, depth: i32) -> i32 {
        let eval = match self.probe_hash(board) {
            Some(x) => x,
            None => {
                let eval = evaluation(board, alpha, beta);
                eval
            }
        };
        if board.status() != BoardStatus::Ongoing {
            return eval;
        }
        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval
        }
        for mv in filter_moves(board, MoveGen::new_legal(&board), depth) {
            let eval = -self.quiescence_search(&board.make_move_new(mv), -beta, -alpha, depth + 1);
            if eval >= beta {
                return beta;
            }
            if eval > alpha {
                alpha = eval;
            }
        }
        self.save_hash(board, alpha);
        alpha
    }

    pub fn probe_hash(&self, board: &Board) -> Option<i32> {
        // Returns the value in the hash for a given board
        let hash = board.get_hash();
        match self.cache.get(hash) {
            None => None,
            Some(x) => {
                if board.side_to_move() == Color::White {
                    Some(x)
                } else {
                    Some(-x)
                }
            }
        }
    }

    fn save_hash(&mut self, board: &Board, eval: i32) {
        // Saves a board and its evaluation in the hash
        let hash = board.get_hash();
        let entry = if board.side_to_move() == Color::White {
            eval
        } else {
            -eval
        };
        self.cache.add(hash, entry);
    }
}
