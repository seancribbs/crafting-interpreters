use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("[line {line}] Error: {message}")]
    Syntax { line: usize, message: &'static str },
}

#[allow(dead_code)]
impl Error {
    pub(crate) fn new(line: usize, message: &'static str) -> Self {
        Self::Syntax { line, message }
    }
}
