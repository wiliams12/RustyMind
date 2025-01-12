use regex::Regex;
use std::io::{self, Write};

use super::engine::Game;

// Handlers for command line commands

pub fn id(console: &mut io::StdoutLock) {
    writeln!(console, "id name Rusty Mind 1.0")
        .unwrap_or_else(|_| panic!("Error writing to the standard output"));
    writeln!(console, "id author Vilém Učík")
        .unwrap_or_else(|_| panic!("Error writing to the standard output"));
}

pub fn options(console: &mut io::StdoutLock) {
    writeln!(console, "option name Depth type spin default 2 min 1")
        .unwrap_or_else(|_| panic!("Error writing to the standard output"));
}

pub fn set_up(game: &mut Game, input: &str, console: &mut io::StdoutLock) {
    let re = Regex::new(r"^position\s+(fen\s+([^ ]+ [^ ]+ [^ ]+ [^ ]+ [^ ]+ [^ ]+)|startpos)(?:\s+moves((\s+\S+)+))?$").unwrap();
    let fen: &str;
    let mut moves = String::new();

    if let Some(captures) = re.captures(input) {
        if let Some(fen_capture) = captures.get(2) {
            fen = fen_capture.as_str();
        } else {
            fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        }

        if let Some(moves_capture) = captures.get(3) {
            moves.push_str(moves_capture.as_str().trim());
        }
    } else {
        writeln!(console, "Internal command error").unwrap();
        return;
    }

    let move_list: Vec<&str> = if !moves.is_empty() {
        moves.split_whitespace().collect()
    } else {
        Vec::new()
    };

    game.set_board(fen, move_list);
}

pub fn set_depth(game: &mut Game, input: &str, console: &mut io::StdoutLock) {
    let words: Vec<&str> = input.split_whitespace().collect();
    let value = words.last().unwrap().parse::<i32>().unwrap();
    if words.len() > 5 || value < 1 {
        writeln!(console, "Invalid value").unwrap();
        return;
    }
    game.set_depth(value);
}

pub fn search(game: &mut Game, console: &mut io::StdoutLock) -> () {
    let best_move = game.play_display();
    writeln!(console, "bestmove {}", best_move).unwrap();
}
