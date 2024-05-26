use std::io::{self, BufRead, Write};
use rusty_mind as rm;
use regex::Regex;
use std::thread;

fn main() {
    // Command line program
    let mut handle = io::stdin().lock();
    let mut out = io::stdout().lock();
    let mut game = rm::Game::new();
    loop {
        //TODO replace all unwrap()s
        let mut input = String::new();
        handle.read_line(&mut input).unwrap();
        let command = input.trim();
        writeln!(out, "Command: {}", command).unwrap();
        out.flush().unwrap();
        match command {
            "uci" => {
                rm::id(&mut out);
                // option to change the depth
                rm::options(&mut out);
                writeln!(out, "uciok").unwrap();
            },
            //TODO maybe implement
            "debug" => (),
            "isready" => writeln!(out, "readyok").unwrap(),
            //TODO implement properly
            // some other commands after the go, try not to implement them
            // return "bestmove <move>"
            //"go" => rm::search(),
            //TODO maybe implement
            "register" => (),
            //TODO maybe implement
            "ucinewgame" => (),
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
                    //thread::spawn(move || {
                    //    game = rm::search(game.clone(), &mut out);
                    //});
                    rm::search(&mut game, &mut out);
                } else {
                    writeln!(out, "Unknown command: {}", command).unwrap();
                }
            }
        }
    }
}
