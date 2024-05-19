use std::{io, str::FromStr};
use shakmaty::{fen::Fen, CastlingMode, Chess, FromSetup, Position};
use chess::{Board, ChessMove, File, Game, GameResult, Piece, Rank};
use rusty_mind::*;
use shakmaty_syzygy::{Tablebase, MaybeRounded, Wdl, Dtz, Syzygy};


fn main() {
    // Command line program
    let mut engine = Engine::new();
    use std::time::Instant;
    let now = Instant::now();
    simulate_game(&mut engine);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn simulate_game(engine: &mut Engine){
    let mut game = Game::new();
    let mut pgn = Pgn::new();
    let mut counter = 0;
    println!("Game starts!");
    loop {
        match game.result() {
            Some(x) => {
            println!("{:?}", x);
            break
            },
            None => ()
        }
        let mv = match engine.negamax_root(&game.current_position(), 2) {
            None => {
                println!("Game ended");
                break
            }
            Some(mv) => mv
        };
        pgn.add_move(mv, &game.current_position());
        game.make_move(mv);
        counter += 1;
        println!("Half moves: {}", counter);
        if game.can_declare_draw() {
            game.declare_draw();
        }
    }
    println!("{}",pgn.get());
}

struct Pgn {
    moves: Vec<String>,
}

impl Pgn {
    fn new() -> Pgn {
        Pgn { moves: vec![] }
    }

    fn add_move(&mut self, mv: ChessMove, board: &Board) {
        // Simplified version only for the move displaying usage
        let src_file = match mv.get_source().get_file() {
            File::A => "a",
            File::B => "b",
            File::C => "c",
            File::D => "d",
            File::E => "e",
            File::F => "f",
            File::G => "g",
            File::H => "h",
        };
        let src_rank = match mv.get_source().get_rank() {
            Rank::First => "1",
            Rank::Second => "2",
            Rank::Third => "3",
            Rank::Fourth => "4",
            Rank::Fifth => "5",
            Rank::Sixth => "6",
            Rank::Seventh => "7",
            Rank::Eighth => "8",
        };
        let dest_file = match mv.get_dest().get_file() {
            File::A => "a",
            File::B => "b",
            File::C => "c",
            File::D => "d",
            File::E => "e",
            File::F => "f",
            File::G => "g",
            File::H => "h",
        };
        let dest_rank = match mv.get_dest().get_rank() {
            Rank::First => "1",
            Rank::Second => "2",
            Rank::Third => "3",
            Rank::Fourth => "4",
            Rank::Fifth => "5",
            Rank::Sixth => "6",
            Rank::Seventh => "7",
            Rank::Eighth => "8",
        };
        let mut piece = match board.piece_on(mv.get_source()) {
            Some(piece) => match piece {
                Piece::Pawn => "",
                Piece::Rook => "R",
                Piece::Knight => "N",
                Piece::Bishop => "B",
                Piece::Queen => "Q",
                Piece::King => "K",
            },
            None => panic!("problem with the pgn")
        };
        let capture = if rusty_mind::is_capture(&mv, board) {"x"} else {""};
        if capture != "" && piece == "" {
            piece = src_file;
        }
        let check = if gives_check(&mv, board) {"+"} else {""};
        let formated = piece.to_string() + capture + dest_file + dest_rank + check;
        self.moves.push(formated);
    }

    fn get(&self) -> String {
        let mut result = String::new();
        let mut move_number = 1;
        for (index, mv) in self.moves.iter().enumerate() {
            if index % 2 == 0 {
                result.push_str(&format!("{}. ", move_number));
                move_number += 1;
            }
            result.push_str(mv);
            result.push(' ');
        }
        result
    }
}


fn test_syzygy() {
    let mut tables: Tablebase<Chess> = Tablebase::new();
    tables.add_directory("../3-4-5_pieces_Syzygy/3-4-5").unwrap();
    
    // let pos: Chess = "8/3k4/8/8/3K4/3P4/8/8 w - - 4 3"
    //     .parse::<Fen>()
    //     .unwrap()
   //      .into_position(CastlingMode::Standard)
    //     .unwrap();
    
    // let dtz = tables.probe_dtz(&pos);

    let pos: Chess = "8/3k4/8/8/3K4/3P4/8/8 w - - 4 3"
    .parse::<Fen>().unwrap()
    .into_position(CastlingMode::Standard).unwrap();

    let dtz = tables.probe_dtz(&pos).unwrap();
    println!("{:?}", dtz);
}