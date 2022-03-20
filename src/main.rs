extern crate clap;
extern crate rustyline;

mod lib;
use lib::engine::Engine;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::{arg, Command};

struct Repl {
    engine: Engine,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            engine: Engine::new(),
        }
    }

    pub fn run(&mut self) {
        let mut rl = Editor::<()>::new();

        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        loop {
            let readline = rl.readline("(lex) λ ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    let mut l = line.clone();
                    match self.engine.parse(l) {
                        Ok(token) => println!("{:?}", token),
                        Err(e) => println!("{}", e),
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        rl.save_history("history.txt").unwrap();
    }
}

fn main() {
    let matches = Command::new("lex")
        .arg(arg!(<PATH>..."file path"))
        .get_matches();

    match matches.subcommand() {
        _ => {
            let p = matches.value_of("PATH");

            match p {
                Some(path) => {
                    let file = std::fs::read_to_string(path).unwrap();
                    match Engine::new().parse(file.to_owned()) {
                        Err(err) => println!("{:?}", err),
                        _ => (),
                    }
                }
                None => {
                    Repl::new().run();
                }
            }
        }
    };
}
