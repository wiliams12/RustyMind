use chess::{get_bishop_moves, get_knight_moves, get_pawn_attacks, get_rook_moves, BitBoard, Board, BoardStatus, Color, Piece, Square, ALL_COLORS, EMPTY, File, Rank};
use super::helpers::is_endgame;
use super::tables::*;
use once_cell::sync::Lazy;


// TODO Fix
static CENTER: Lazy<BitBoard> = Lazy::new(|| {
    BitBoard::from_square(Square::D4) &
    BitBoard::from_square(Square::D5) &
    BitBoard::from_square(Square::E4) &
    BitBoard::from_square(Square::E5)
});

pub fn evaluation(board: &Board, alpha: i32, beta: i32) -> i32 {
    match board.status() {
        BoardStatus::Checkmate => return -999999,
        BoardStatus::Stalemate => return 0,
        BoardStatus::Ongoing => ()
    }
    let mut eval = 0;
    for color in ALL_COLORS {
        let pieces = *board.color_combined(color);
        for square in pieces {
            eval += evaluate_piece(board, square, color, board.side_to_move())
        }
    }
    // Lazy evaluation
    // 80 is the 'max' player gets in a material equal position + some little margin
    let margin = 80;
    if eval > beta - margin || eval < alpha + margin {
        // center can be used only with cache, it would be to slow otherwise
        eval += mobility(board, Color::White) + center_control(board, Color::White);
        eval -= mobility(board, Color::Black) + center_control(board, Color::Black);
        eval += evaluate_piece(board, board.king_square(Color::White), Color::White, board.side_to_move());
        eval -= evaluate_piece(board, board.king_square(Color::Black), Color::Black, board.side_to_move());
    }
    eval
}

fn center_control(board: &Board, color: Color) -> i32 {
    let mut eval = 0;
    let pieces = board.combined();
    for piece in *board.pieces(Piece::Bishop) {
        // Should always be Some(_)
        if board.color_on(piece).unwrap() == color {
            if (get_bishop_moves(piece, *pieces) & *CENTER) != EMPTY {
                eval += 5;
            }
        }
    }
    for piece in *board.pieces(Piece::Knight) {
        // Should always be Some(_)
        if board.color_on(piece).unwrap() == color {
            if (get_knight_moves(piece) & *CENTER) != EMPTY {
                eval += 5;
            }
        }
    }
    for piece in *board.pieces(Piece::Rook) {
        // Should always be Some(_)
        if board.color_on(piece).unwrap() == color {
            if (get_rook_moves(piece, *pieces) & *CENTER) != EMPTY {
                eval += 5;
            }
        }
    }
    for piece in *board.pieces(Piece::Queen) {
        // Should always be Some(_)
        if board.color_on(piece).unwrap() == color {
            if (get_rook_moves(piece, *pieces) & *CENTER) != EMPTY  || (get_bishop_moves(piece, *pieces) & *CENTER) != EMPTY {
                eval += 5;
            }
        }
    }
    eval
}

fn mobility(board: &Board, color: Color) -> i32 {
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

fn evaluate_piece(board: &Board, square: Square, color: Color, turn: Color) -> i32{
    let turn_multiplier = if turn == Color::White { 1 } else { -1 };
    let color_multiplier = if color == Color::White { 1 } else { -1 };
    let piece = board.piece_on(square).unwrap();
    (match piece {
        // Higher pawn value because pawns don't have mobility so their actual value would be lower relative to other pieces
        Piece::Pawn => 115 + read_table(&PAWN_TABLE, square, color),
        Piece::Knight => 300 + KNIGHT_TABLE[square.to_int() as usize],
        Piece::Bishop => 310 + BISHOP_TABLE[square.to_int() as usize],
        Piece::Rook => 500 + read_table(&ROOK_TABLE, square, color),
        Piece::Queen => 900 + QUEEN_TABLE[square.to_int() as usize],
        Piece::King => if is_endgame(board) { KING_TABLE_ENDGAME[square.to_int() as usize] } else {
            read_table(&KING_TABLE, square, color)
        } 
    } * turn_multiplier * color_multiplier)
}

fn read_table(table: &[i32;64], square: Square, color: Color) -> i32 {
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
