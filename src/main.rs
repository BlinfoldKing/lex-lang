extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

struct Repl;

impl Repl {
    pub fn run() {
        let mut rl = Editor::<()>::new();

        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        loop {
            let readline = rl.readline("(lex) λ ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }

        rl.save_history("history.txt").unwrap();
    }
}

fn main() {
    Repl::run()
}
