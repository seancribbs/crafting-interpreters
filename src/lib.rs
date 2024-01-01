use std::{io::Write, path::PathBuf};

pub mod error;
mod interpret;
mod scanner;
mod syntax;

use error::Result;
use scanner::*;
use syntax::*;

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
    let parser = Parser::new(tokens);
    let expr = parser.parse()?;
    interpret::interpret(expr);
    Ok(())
}
