use chess::{Board, ChessMove, MoveGen, Piece, EMPTY, CastleRights, Color, File, Rank, Square};
use shakmaty::{CastlingMode, Chess, fen::Fen};

pub fn sorting_func(board: &Board, chess_move: &ChessMove) -> i32 {
    // returns a numerical value to a move based on the attractivness of the move
    if gives_check(chess_move, board) {
        2
    } else if is_capture(chess_move, board) {
        1
    } else {
        0
    }
}

pub fn reorder_moves(board: &Board, moves: MoveGen) -> Vec<ChessMove>{
    // Returns a reordered list of moves from the most attractive to the least attractive
    let mut new_moves: Vec<ChessMove> = moves.collect();
    new_moves.sort_by_key(|chess_move| sorting_func(board, chess_move));
    new_moves.reverse();
    new_moves
}

pub fn filter_moves(board: &Board, moves: MoveGen, depth: i32) -> Vec<ChessMove> {
    // Filters the given moves, returns only captures and checks (if the current side to move is not in check)
    // Used in the quiescence search
    if board.checkers() != &EMPTY {
        return moves.collect();
    }
    moves.filter(|x| (gives_check(x, board) && depth < 10) || is_capture(x, board)).collect()
}

pub fn is_endgame(board: &Board) -> bool {
    // If number of pieces not including pawns and kings is lower than a given boundary, it returns true
    let mut pieces = *board.combined();
    pieces = pieces ^ board.pieces(Piece::Pawn); 
    pieces = pieces ^ board.pieces(Piece::King); 
    pieces.collect::<Vec<_>>().len() < 4
}

pub fn shakmaty_square(square: shakmaty::Square) -> u8{
    u8::from(square)
}

pub fn square_from_integer(index: u8) -> Square {
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

pub fn shakmaty_board(board: &Board) -> Chess{
    board_to_fen(board)
    .parse::<Fen>().unwrap()
    .into_position(CastlingMode::Standard).unwrap()
}
pub fn is_capture(chess_move: &ChessMove, board: &Board) -> bool{
    // Checks whether a move is a capture
    match board.piece_on(chess_move.get_dest()) {
        None => false,
        Some(_) => true
    }
}

pub fn gives_check(chess_move: &ChessMove, board: &Board) -> bool {
    // Checks whether a move is a check
    board.make_move_new(*chess_move).checkers() != &EMPTY
}


pub fn board_to_fen(board: &Board) -> String{
    // Converts the "chess" crate board into a FEN string because the crate doesn't support it
    // Doesn't keep track of the 50 moves rule
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