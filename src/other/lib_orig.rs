use std::{collections::HashMap, iter::Filter, result, str::FromStr};
use chess::{get_pawn_attacks, BitBoard, Board, BoardStatus, CastleRights, ChessMove, Color, File, Game, MoveGen, Piece, Rank, Square, ALL_COLORS, EMPTY, CacheTable};
use shakmaty_syzygy::{AmbiguousWdl, Dtz, MaybeRounded, Syzygy, Tablebase, Wdl};
use shakmaty::{CastlingMode, Chess, fen::Fen, Move};
use rand::rngs::SmallRng;
use rand::{SeedableRng, thread_rng};
use rand::prelude::*;

// TODO Set up the cache
// TODO optimise the speed somehow

const PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50,  50,  50,  50,  50,  50,  50,  50,
    20,  20,  25,  40,  40,  25,  20,  20,
    0,  0,  10,  30,  30,  10,  0,  0,
   -10, -20,  10,  30,  30,  10, -20, -10,
   -10,  0,  0,  20,  20,  0,  0,  -10,
    10,  10,  0,  -10, -10, 0,  10,  10,
    1,  0,  0,  0,  0,  0,  0,  0,
];


const KNIGHT_TABLE: [i32; 64] = [
    -40, -10, -10, -10, -10, -10, -10, -40,
    -10, 0, 10, 20, 20, 10, 0, -10,
    -10, 0,  10,  10,  10,  10, 0, -10,
    -20, 0,  0,  10,  10,  0, 0, -20,
    -20, 0,  0,  0,  0,  0, 0, -20,
    -20, -10,  10,  0,  0,  10, -10, -20,
    -30, -20, -10, 0, 0, -10, -20, -30,
    -40, -30, -30, -30, -30, -30, -30, -40,
];


const BISHOP_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    0,  0,  0,  0,  0,  0,  0, 0,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -10,  0,  0,  0,  0,  0,  0, -10,
    0,  0,  0,  0,  0,  0,  0, 0,
    -10,  10,  0,  0,  0,  0,  10, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

const ROOK_TABLE: [i32; 64] = [
     0, 10, 10, 10, 10, 10, 10,  0,
     10, 20, 20, 20, 20, 20, 20,  10,
    -10, 0, 0, 0, 0, 0, 0, -10,0
    -10, 0, 0, 0, 0, 0, 0, -10,
    -10, 0, 0, 0, 0, 0, 0, -10,
    -10, 0, 0, 0, 0, 0, 0, -10,
    -10, 0, 0, 0, 0, 0, 0, -10,
     0, 0, 0, 10, 10, 0, 0,  0,
];

const QUEEN_TABLE: [i32; 64] = [
    -20, -10, -10,  0,  0, -10, -10, -20,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -10,  0,  0,  5,  5,  0,  0, 0,
    0,  0,  0,  5,  5,  0,  0, -10,
    -10,  5,  0,  0,  0,  5,  0, -10,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -20, -10, 0,  0,  0, -10, -10, -20,
];
const KING_TABLE: [i32; 64] = [
    -50, -50, -50, -50, -50, -50, -50, -50,
    -50, -50, -50, -50, -50, -50, -50, -50,
    -50, -50, -50, -80, -80, -50, -50, -50,
    -50, -50, -80, -100, -100, -80, -50, -50,
    -50, -50, -80, -100, -100, -80, -50, -50,
    -30, -50, -50, -80, -80, -50, -50, -30,
    -10, -10, -40, -40, -40, -40, -10, -10,
     20,  30,  20,  -10,  -10,  -10,  30,  30,
];
const KING_TABLE_ENDGAME: [i32; 64] = [
    -50, -50, -50, -50, -50, -50, -50, -50,
    -50, -20, -20, -20, -20, -20, -20, -50,
    -50, -20,  20,  20,  20,  20, -20, -50,
    -50, -20,  20,  60,  60,  20, -20, -50,
    -50, -20,  20,  60,  60,  20, -20, -50,
    -50, -20,  20,  20,  20,  20, -20, -50,
    -50, -20, -20, -20, -20, -20, -20, -50,
    -50, -50, -50, -50, -50, -50, -50, -50,
];

const CENTER: [Square; 4] = [Square::D4, Square::D5, Square::E4, Square::E5];

pub struct Engine {
    tablebase: Tablebase<Chess>,
    random: ThreadRng,
    cache: CacheTable<i32>
}

impl Engine {
    pub fn new() -> Engine{
        let mut tables = Tablebase::new();
        tables.add_directory("../3-4-5_pieces_Syzygy/3-4-5").unwrap();
        Engine {
            tablebase: tables,
            random: thread_rng(),
            cache: CacheTable::new(524288, 0)
        }
    }

    pub fn negamax_root(&mut self, board: &Board, depth: i32) -> Option<ChessMove> {
        match board.status() {
            BoardStatus::Checkmate => { 
                println!("checkmate");
                return None;
            },
            BoardStatus::Stalemate => {
                println!("stalemate");
                return None;
            },
            BoardStatus::Ongoing => (),
        }
        if self.is_syzygy(board) {
            return self.convert_endgame(board).1
        } 
        let mut best_move: Option<ChessMove> = None;  
        let mut max_eval = -100000;
        for mv in self.reorder_moves(&board, MoveGen::new_legal(&board)) {
            let mut eval = -self.negamax(&board.make_move_new(mv),-99999, 99999,depth-1);
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
        if depth == 0 || match board.status() {
            BoardStatus::Checkmate => true,
            BoardStatus::Stalemate => true,
            BoardStatus::Ongoing => false
        } {
            return self.quiescence_search(board,alpha,beta, 0);
        }

        if self.is_syzygy(board) {
            return self.convert_endgame(board).0
        }

        for mv in self.reorder_moves(&board, MoveGen::new_legal(&board)) {
            let eval = -self.negamax(&board.make_move_new(mv),-beta,-alpha,depth-1);
            if alpha >= beta{
                break
            }

            if eval > alpha{
                alpha = eval;
            }

        }
        alpha
    }

    fn quiescence_search(&mut self,board: &Board, mut alpha: i32, beta: i32, depth: i32) -> i32 {
        // Syzygy is not in quiescence search because it uses a lot of performance and quiescence search is the most vast tree part
        match board.status() {
            BoardStatus::Checkmate => return -99999,
            BoardStatus::Stalemate => return 0,
            BoardStatus::Ongoing => ()
        }
        // let eval = match self.probe_hash(board) {
        //     Some(x) => return x,
        //     None => self.evaluation(board, alpha, beta)
        // };
        let eval = self.evaluation(board, alpha, beta);
        // limiting the depth
        if eval >= beta{
            return beta;
        }

        if eval > alpha {
            alpha = eval;
        }


        for mv in self.filter_moves(board, MoveGen::new_legal(&board), depth) {
            let eval = -self.quiescence_search(&board.make_move_new(mv),-beta,-alpha, depth+1);
            if eval >= beta{
                return beta;
            }

            if eval > alpha {
                alpha = eval;
            }
        }
        // self.save_hash(board, alpha);
        alpha
    }

    fn evaluation(&self,board: &Board, alpha: i32, beta: i32) -> i32 {
        match board.status() {
            BoardStatus::Checkmate => return -999999,
            BoardStatus::Stalemate => return 0,
            BoardStatus::Ongoing => ()
        }
        let mut eval = 0;
        for color in ALL_COLORS {
            let pieces = *board.color_combined(color);
            for square in pieces {
                eval += self.evaluate_piece(board, square, color, board.side_to_move())
            }
        }
        // Lazy evaluation
        // adjust margin based on evaluation complexness
        // 40 is the 'max' player gets in a material equal position
        // ? increase ?
        let margin = 40;
        if eval > beta - margin || eval < alpha + margin {
            // center is not used because it is really slow
            eval += self.mobility(board, Color::White); // + self.center_control(board, Color::White);
            eval -= self.mobility(board, Color::Black); // + self.center_control(board, Color::Black);
            eval += self.evaluate_piece(board, board.king_square(Color::White), Color::White, board.side_to_move());
            eval -= self.evaluate_piece(board, board.king_square(Color::Black), Color::Black, board.side_to_move());
        }
        eval
    }

    fn center_control(&self, board: &Board, color: Color) -> i32 {
        let mut eval = 0;
        for mv in MoveGen::new_legal(&board) {
            if CENTER.contains(&mv.get_dest()) {
                eval += 5;
            }
        }
        eval
    }

    fn mobility(&self, board: &Board, color: Color) -> i32 {
        // Returns evaluation of the mobility relative to the selected side
        let mut eval = 0;
        if color != board.side_to_move() {
            let board = &board.clone();
            board.null_move();
        }
        // Pawns don't count as pieces thus their "mobility" is irrelevant
        // King is not included for it would encourage him to leave shelter
        let bishops = *board.pieces(Piece::Bishop) & board.color_combined(color);
        let knights = *board.pieces(Piece::Bishop) & board.color_combined(color);
        let rooks = *board.pieces(Piece::Rook) & board.color_combined(color);
        let queens = *board.pieces(Piece::Queen) & board.color_combined(color);
        let enemy_pawns = *board.pieces(Piece::Pawn) & board.color_combined(!color);
        let mut attacks_on_bishop = EMPTY;
        for pawn in enemy_pawns {
            attacks_on_bishop = attacks_on_bishop & get_pawn_attacks(pawn, !color, bishops)
        }
        eval += (bishops ^ attacks_on_bishop).collect::<Vec<_>>().len()*5;

        let mut attacks_on_knight = EMPTY;
        for pawn in enemy_pawns {
            attacks_on_knight = attacks_on_knight & get_pawn_attacks(pawn, !color, knights)
        }
        eval += (bishops ^ attacks_on_knight).collect::<Vec<_>>().len()*3;

        let mut attacks_on_rook = EMPTY;
        for pawn in enemy_pawns {
            attacks_on_rook = attacks_on_rook & get_pawn_attacks(pawn, !color, rooks);
        }
        eval += (bishops ^ attacks_on_rook).collect::<Vec<_>>().len()*3;

        let mut attacks_on_queen = EMPTY;
        for pawn in enemy_pawns {
            attacks_on_queen = attacks_on_queen & get_pawn_attacks(pawn, !color, queens);
        }
        eval += (bishops ^ attacks_on_queen).collect::<Vec<_>>().len();

        eval as i32
    }

    fn evaluate_piece(&self,board: &Board, square: Square, color: Color, turn: Color) -> i32{
        let turn_multiplier = if turn == Color::White { 1 } else { -1 };
        let color_multiplier = if color == Color::White { 1 } else { -1 };
        let piece = board.piece_on(square).unwrap();
        (match piece {
            // Higher pawn value because pawns don't have mobility
            // TODO If pawn eval was implemented, set back to 100
            Piece::Pawn => 115 + self.read_table(&PAWN_TABLE, square, color),
            Piece::Knight => 300 + KNIGHT_TABLE[square.to_int() as usize],
            Piece::Bishop => 310 + BISHOP_TABLE[square.to_int() as usize],
            Piece::Rook => 500 + self.read_table(&ROOK_TABLE, square, color),
            Piece::Queen => 900 + QUEEN_TABLE[square.to_int() as usize],
            Piece::King => 0     
        } * turn_multiplier * color_multiplier)
    }

    fn probe_hash(&self, board: &Board) -> Option<i32>{
        let hash = board.get_hash();
        match self.cache.get(hash) {
            None => None,
            Some(x) => if board.side_to_move() == Color::White { Some(x) } else { Some(-x) }
        }
    }

    fn save_hash(&mut self, board: &Board, eval: i32) {
        let hash = board.get_hash();
        let entry = if board.side_to_move() == Color::White { eval } else { -eval };
        self.cache.add(hash, entry);
    }

    fn read_table(&self,table: &[i32;64], square: Square, color: Color) -> i32 {
        if color == Color::Black { 
            table[square.to_int() as usize] 
        } 
        else { 
            let file = match square.get_file() {
                File::A => 0,
                File::B => 1,
                File::C => 2,
                File::D => 3,
                File::E => 4,
                File::F => 5,
                File::G => 6,
                File::H => 7,
            };
            
            let rank = match square.get_rank() {
                Rank::First => 7,
                Rank::Second => 6,
                Rank::Third => 5,
                Rank::Fourth => 4,
                Rank::Fifth => 3,
                Rank::Sixth => 2,
                Rank::Seventh => 1,
                Rank::Eighth => 0,
            };
            
            table[8 * rank as usize + file as usize] 
        }
    }

    fn sorting_func(&self, board: &Board, chess_move: &ChessMove) -> i32 {
        if gives_check(chess_move, board) {
            2
        } else if is_capture(chess_move, board) {
            1
        } else {
            0
        }
    }

    fn reorder_moves(&self,board: &Board, moves: MoveGen) -> Vec<ChessMove>{
        let mut new_moves: Vec<ChessMove> = moves.collect();
        new_moves.sort_by_key(|chess_move| self.sorting_func(board, chess_move));
        new_moves.reverse();
        new_moves
    }

    fn filter_moves(&self, board: &Board, moves: MoveGen, depth: i32) -> Vec<ChessMove> {
        if board.checkers() != &EMPTY {
            return moves.collect();
        }
        moves.filter(|x| (gives_check(x, board) && depth < 10) || is_capture(x, board)).collect()
    }

    pub fn is_syzygy(&self, board: &Board) -> bool {
        board.combined().collect::<Vec<_>>().len() < 6
    }

    fn is_endgame(&self, board: &Board) -> bool {
        // If number of pieces not including 
        let mut pieces = *board.combined();
        pieces = pieces ^ board.pieces(Piece::Pawn); 
        pieces = pieces ^ board.pieces(Piece::King); 
        pieces.collect::<Vec<_>>().len() < 2
    }

    pub fn convert_endgame(&self, board: &Board) -> (i32, Option<ChessMove>){
        match board.status() {
            BoardStatus::Checkmate => return (-99999,None),
            BoardStatus::Stalemate => return (0,None),
            BoardStatus::Ongoing => ()
        };
        let pos = &shakmaty_board(board);
        let mv = self.tablebase.best_move(pos).unwrap().unwrap().0;
        let wdl = self.tablebase.probe_wdl(pos).unwrap();
        let mv = Some(match mv {
            Move::Normal { from, to, promotion, role, capture } => {
                let source = square_from_integer(shakmaty_square(from));
                let dest = square_from_integer(shakmaty_square(to));
                let promotion_ = match promotion {
                    None => None,
                    Some(x) => Some(match u32::from(x) {
                        1 => Piece::Pawn,
                        2 => Piece::Knight,
                        3 => Piece::Bishop,
                        4 => Piece::Rook,
                        5 => Piece::Queen,
                        6 => Piece::King,
                        _ => unimplemented!(), // Handle other cases if needed
                    }),
                };
                ChessMove::new(source, dest, promotion_)
            },
            Move::EnPassant {from, to} => {
                let source = square_from_integer(shakmaty_square(from));
                let dest = square_from_integer(shakmaty_square(to));
                ChessMove::new(source, dest, None)
            },
            // Those 2 are not implemented because I am lazy
            Move::Castle {king, rook}=> panic!("Lazyness error"),
            Move::Put {role, to} => panic!("Lazyness error"),
        });

        if wdl.signum() > 0 {
            return (99999, mv)
        }
        else if wdl.signum() < 0 {
            return (-99999, mv)
        }
        else {
            return (0, mv)
        }

    }
}

fn shakmaty_square(square: shakmaty::Square) -> u8{
    u8::from(square)
}

fn square_from_integer(index: u8) -> Square {
    // TODO handle errors better
    let rank = index / 8;
    let file = index - rank * 8;
    let rank = match rank {
        0 => Rank::First,
        1 => Rank::Second,
        2 => Rank::Third,
        3 => Rank::Fourth,
        4 => Rank::Fifth,
        5 => Rank::Sixth,
        6 => Rank::Seventh,
        7 => Rank::Eighth,
        _ => {
            panic!("Invalid rank index");
        }
    };
    let file = match file {
        0 => File::A,
        1 => File::B,
        2 => File::C,
        3 => File::D,
        4 => File::E,
        5 => File::F,
        6 => File::G,
        7 => File::H,
        _ => {
            panic!("Invalid file index");
        }
    };
    Square::make_square(rank, file)
}

fn shakmaty_board(board: &Board) -> Chess{
    board_to_fen(board)
    .parse::<Fen>().unwrap()
    .into_position(CastlingMode::Standard).unwrap()
}
pub fn is_capture(chess_move: &ChessMove, board: &Board) -> bool{
    match board.piece_on(chess_move.get_dest()) {
        None => false,
        Some(_) => true
    }
}

pub fn gives_check(chess_move: &ChessMove, board: &Board) -> bool {
    board.make_move_new(*chess_move).checkers() != &EMPTY
}


pub fn board_to_fen(board: &Board) -> String{
    // Should be working properly
    let mut string = String::new();
    for rank in (0..8).rev() {
        let mut counter: u8 = 0;
        for file in 0..8 {
            let rank_ = match rank {
                0 => Rank::First,
                1 => Rank::Second,
                2 => Rank::Third,
                3 => Rank::Fourth,
                4 => Rank::Fifth,
                5 => Rank::Sixth,
                6 => Rank::Seventh,
                7 => Rank::Eighth,
                _ => {
                    eprintln!("Invalid rank index");
                    break;
                }
            };
            let file_ = match file {
                0 => File::A,
                1 => File::B,
                2 => File::C,
                3 => File::D,
                4 => File::E,
                5 => File::F,
                6 => File::G,
                7 => File::H,
                _ => {
                    eprintln!("Invalid file index");
                    break;
                }
            };
            let square = Square::make_square(rank_, file_);
            match board.piece_on(square) {
                None => counter += 1,
                Some(piece) => {
                    if counter != 0 {
                        string.push_str(&counter.to_string());
                        counter = 0;
                    }
                    let piece_char = match piece {
                        Piece::Pawn => 'p',
                        Piece::Rook => 'r',
                        Piece::Knight => 'n',
                        Piece::Bishop => 'b',
                        Piece::Queen => 'q',
                        Piece::King => 'k',
                    };
                    if board.color_on(square).unwrap() == Color::White {
                        string.push(piece_char.to_ascii_uppercase());
                    }
                    else {
                        string.push(piece_char)
                    }
                }
            }
        }
        if counter != 0 {
            string.push_str(&counter.to_string());
        }
        string.push('/');
    }
    string.pop();
    string.push(' ');
    string.push(match board.side_to_move() {
        Color::Black => 'b',
        Color::White => 'w'
    });
    string.push(' ');
    let white = match board.castle_rights(Color::White) {
        CastleRights::Both => "KQ",
        CastleRights::KingSide => "K",
        CastleRights::QueenSide => "Q",
        CastleRights::NoRights => ""
    };
    let black = match board.castle_rights(Color::Black) {
        CastleRights::Both => "kq",
        CastleRights::KingSide => "k",
        CastleRights::QueenSide => "q",
        CastleRights::NoRights => ""
    };
    if black == "" && white == "" {
        string.push('-');
    }
    else {
        string.push_str(white);
        string.push_str(black);
    }

    match board.en_passant() {
        None => string.push_str(" - "),
        Some(square) => {
            let file = match square.get_file() {
                File::A => "a",
                File::B => "b",
                File::C => "c",
                File::D => "d",
                File::E => "e",
                File::F => "f",
                File::G => "g",
                File::H => "h",
            };
            let rank = match square.get_rank() {
                Rank::First => "1",
                Rank::Second => "2",
                Rank::Third => "3",
                Rank::Fourth => "4",
                Rank::Fifth => "5",
                Rank::Sixth => "6",
                Rank::Seventh => "7",
                Rank::Eighth => "8",
            };
            let formatted = format!("{}{}", file, rank);
            string.push_str(&formatted);
        }
    }
    // Not implemented becuase it is not that important and it would decrease the clarity of the code
    string.push_str(&format!("0 0"));

    string
}