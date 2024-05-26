use std::io::{self, BufRead, Write};
use rusty_mind as rm;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdin.lock();
    let mut out = stdout.lock();
    let mut game = rm::Game::new();
    writeln!(out, "Rusty Mind 0.1").unwrap();

    loop {
        let mut input = String::new();
        if handle.read_line(&mut input).is_err() {
            writeln!(out, "Failed to read line").unwrap();
            out.flush().unwrap();
            continue;
        }

        let command = input.trim();
        writeln!(out, "Command: {}", command).unwrap();
        out.flush().unwrap();

        match command {
            "uci" => {
                rm::id(&mut out);
                rm::options(&mut out);
                writeln!(out, "uciok").unwrap();
                out.flush().unwrap();
            },
            "isready" => {
                writeln!(out, "readyok").unwrap();
                out.flush().unwrap();
            },
            "ucinewgame" => {
                game = rm::Game::new(); // Reset the game
                writeln!(out, "new game started").unwrap();
                out.flush().unwrap();
            },
            "quit" => break,
            _ => {
                let re1 = Regex::new(r"setoption name Depth value (\d+)").unwrap();
                let re2 = Regex::new(r"^position\s+(fen\s+([^ ]+ [^ ]+ [^ ]+ [^ ]+ [^ ]+ [^ ]+)|startpos)(?:\s+moves(\s+\S+)+)?").unwrap();
                let re3 = Regex::new(r"^go(?:\s.*)?$").unwrap();

                if re1.is_match(command) {
                    rm::set_depth(&mut game, command, &mut out);
                } else if re2.is_match(command) {
                    rm::set_up(&mut game, command, &mut out);
                } else if re3.is_match(command) {
                    // Implement the search directly without threading for now
                    rm::search(&mut game, &mut out);
                } else {
                    writeln!(out, "Unknown command: {}", command).unwrap();
                    out.flush().unwrap();
                }
            }
        }
    }
}
