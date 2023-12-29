use std::{io::Write, path::PathBuf};

pub mod error;
mod scanner;

use error::Result;
use scanner::*;

pub fn run_file(path: impl Into<PathBuf>) -> Result<()> {
    let source = std::fs::read_to_string(path.into())?;
    run(source)
}

pub fn run_prompt() {
    loop {
        let mut line = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        if std::io::stdin().read_line(&mut line).is_ok() {
            if let Err(err) = run(line) {
                eprintln!("{err}");
            }
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    println!();
    for token in tokens {
        println!("{token}");
    }
    Ok(())
}
