use super::helpers::is_endgame;
use super::tables::*;
use chess::{
    get_bishop_moves, get_knight_moves, get_pawn_attacks, get_rook_moves, BitBoard, Board,
    BoardStatus, Color, File, Piece, Rank, Square, ALL_COLORS, EMPTY,
};
use once_cell::sync::Lazy;

static CENTER: Lazy<BitBoard> = Lazy::new(|| {
    BitBoard::from_square(Square::D4)
        & BitBoard::from_square(Square::D5)
        & BitBoard::from_square(Square::E4)
        & BitBoard::from_square(Square::E5)
});

pub fn evaluation(board: &Board, alpha: i32, beta: i32) -> i32 {
    // Evaluation function which returns an evaluation relative to the side to move
    // Returns a value if the game has ended
    match board.status() {
        BoardStatus::Checkmate => return -99_999,
        BoardStatus::Stalemate => return 0,
        BoardStatus::Ongoing => (),
    }
    let mut eval = 0;
    // Iterates over all the pieces on the board
    for color in ALL_COLORS {
        let pieces = *board.color_combined(color);
        for square in pieces {
            eval += evaluate_piece(board, square, color, board.side_to_move())
        }
    }

    // Lazy evaluation
    // Executes only if the position is promising
    // 80 is the 'max' player gets in a material equal position + some little margin
    let margin = 80;
    if eval + margin < beta && eval + margin > alpha {
        // center can be used only with cache, it would be to slow otherwise
        eval += mobility(board, Color::White) + center_control(board, Color::White);
        eval -= mobility(board, Color::Black) + center_control(board, Color::Black);
    }
    eval
}

fn center_control(board: &Board, color: Color) -> i32 {
    // Checks if pieces control the center
    // Not implemented for the king and the pawns
    // King shouldn't be encouraged to be in the center
    // Pawns are not implemented because the crate generates only the real moves in a position.
    // Pawn moving into center doesn't mean it controls it
    // Pawn should be attacking the center which would have to be done by creating a new board with imaginary pieces in the center and that would be to computationaly complex
    // Pawns have a bonus for being in the center so this part isn't completely omitted
    let mut eval = 0;
    // "pieces" are needed because the crate's move generation functions demand the possible obsticles
    let pieces = board.combined();
    for piece in *board.pieces(Piece::Bishop) {
        // Should always be Some(_)
        if board.color_on(piece).unwrap() == color {
            // Checks the bishop moves
            if (get_bishop_moves(piece, *pieces) & *CENTER) != EMPTY {
                eval += 5;
            }
        }
    }
    for piece in *board.pieces(Piece::Knight) {
        // Should always be Some(_)
        if board.color_on(piece).unwrap() == color {
            // Checks the knight moves
            if (get_knight_moves(piece) & *CENTER) != EMPTY {
                eval += 5;
            }
        }
    }
    for piece in *board.pieces(Piece::Rook) {
        // Should always be Some(_)
        if board.color_on(piece).unwrap() == color {
            // Checks the rook moves
            if (get_rook_moves(piece, *pieces) & *CENTER) != EMPTY {
                eval += 5;
            }
        }
    }
    for piece in *board.pieces(Piece::Queen) {
        // Should always be Some(_)
        if board.color_on(piece).unwrap() == color {
            // Checks the queen moves
            if (get_rook_moves(piece, *pieces) & *CENTER) != EMPTY
                || (get_bishop_moves(piece, *pieces) & *CENTER) != EMPTY
            {
                eval += 5;
            }
        }
    }
    eval
}

fn mobility(board: &Board, color: Color) -> i32 {
    // Returns evaluation of the mobility relative to the selected side
    let mut eval = 0;
    let board_clone: Board;

    // Makes a null move if it's not the turn of the selected color
    // The board needs to be cloned because the crate erases en passants when doing a null move
    let board = if color != board.side_to_move() {
        board_clone = board.clone();
        board_clone.null_move();
        &board_clone
    } else {
        board
    };
    // Pawns don't count as pieces thus their "mobility" is irrelevant
    // King is not included for it would encourage him to leave shelter
    let bishops = *board.pieces(Piece::Bishop) & board.color_combined(color);
    let knights = *board.pieces(Piece::Bishop) & board.color_combined(color);
    let rooks = *board.pieces(Piece::Rook) & board.color_combined(color);
    let queens = *board.pieces(Piece::Queen) & board.color_combined(color);
    let enemy_pawns = *board.pieces(Piece::Pawn) & board.color_combined(!color);
    // Finds the intersection of bishop moves and opponents pawn attacks and then counts only the squares that are unprotected by a pawn
    // The reward for possible squares is as follows:
    // Bishop and Rooks: 3 centipawns
    // Queen: 1 centipawn (She has the biggest mobility and her mobility is not that important)
    // Knight: 5 centipawns (It has the least mobility and its mobility is very important)
    let mut attacks_on_bishop = EMPTY;
    for pawn in enemy_pawns {
        attacks_on_bishop = attacks_on_bishop & get_pawn_attacks(pawn, !color, bishops)
    }
    eval += (bishops ^ attacks_on_bishop).collect::<Vec<_>>().len() * 3;

    let mut attacks_on_knight = EMPTY;
    for pawn in enemy_pawns {
        attacks_on_knight = attacks_on_knight & get_pawn_attacks(pawn, !color, knights)
    }
    eval += (knights ^ attacks_on_knight).collect::<Vec<_>>().len() * 5;

    let mut attacks_on_rook = EMPTY;
    for pawn in enemy_pawns {
        attacks_on_rook = attacks_on_rook & get_pawn_attacks(pawn, !color, rooks);
    }
    eval += (rooks ^ attacks_on_rook).collect::<Vec<_>>().len() * 3;

    let mut attacks_on_queen = EMPTY;
    for pawn in enemy_pawns {
        attacks_on_queen = attacks_on_queen & get_pawn_attacks(pawn, !color, queens);
    }
    eval += (queens ^ attacks_on_queen).collect::<Vec<_>>().len();

    eval as i32
}

fn evaluate_piece(board: &Board, square: Square, color: Color, turn: Color) -> i32 {
    // Returns the material value of a piece and its value according to a placement table
    let turn_multiplier = if turn == Color::White { 1 } else { -1 };
    let color_multiplier = if color == Color::White { 1 } else { -1 };
    let piece = board.piece_on(square).unwrap();
    (match piece {
        // Higher pawn value because pawns don't have mobility so their actual value would be lower relative to other pieces
        // Read table is called only on the pieces that require it
        Piece::Pawn => 115 + read_table(&PAWN_TABLE, square, color),
        Piece::Knight => 300 + KNIGHT_TABLE[square.to_int() as usize],
        Piece::Bishop => 310 + BISHOP_TABLE[square.to_int() as usize],
        Piece::Rook => 500 + read_table(&ROOK_TABLE, square, color),
        Piece::Queen => 900 + QUEEN_TABLE[square.to_int() as usize],
        Piece::King => {
            if is_endgame(board) {
                KING_TABLE_ENDGAME[square.to_int() as usize]
            } else {
                read_table(&KING_TABLE, square, color)
            }
        }
    } * turn_multiplier
        * color_multiplier)
}

fn read_table(table: &[i32; 64], square: Square, color: Color) -> i32 {
    // Reads the placement table according to the side to move
    if color == Color::Black {
        table[square.to_int() as usize]
    } else {
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
