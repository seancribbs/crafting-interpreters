use lox::*;

fn main() -> Result<(), lox::error::Error> {
    let mut args = std::env::args();
    match args.len() {
        l if l > 2 => {
            eprintln!("Usage: {} [script]", args.next().unwrap());
            std::process::exit(64);
        }
        2 => run_file(args.nth(1).unwrap())?,
        _ => run_prompt(),
    }
    Ok(())
}
