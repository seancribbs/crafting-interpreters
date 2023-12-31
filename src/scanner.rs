use phf::phf_map;
use std::fmt::Display;

use crate::error::{Error, Result};

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two-character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Not a lexeme, but for parser simplicity
    Eof,
}

impl TokenType {
    pub fn matches(&self, other: &Self) -> bool {
        use std::mem::discriminant;
        discriminant(self) == discriminant(other)
    }
}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

#[derive(Clone, Debug)]
pub struct Token {
    pub(crate) ty: TokenType,
    pub(crate) lexeme: String,
    pub(crate) line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.ty, self.lexeme)
    }
}

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

const SKIP_TOKEN: Result<Option<Token>> = Ok(None);

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token {
            ty: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
        });
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>> {
        let Some(c) = self.advance() else {
            return Ok(None);
        };
        let token = match c {
            '(' => self.token(TokenType::LeftParen),
            ')' => self.token(TokenType::RightParen),
            '{' => self.token(TokenType::LeftBrace),
            '}' => self.token(TokenType::RightBrace),
            ',' => self.token(TokenType::Comma),
            '.' => self.token(TokenType::Dot),
            '-' => self.token(TokenType::Minus),
            '+' => self.token(TokenType::Plus),
            ';' => self.token(TokenType::Semicolon),
            '*' => self.token(TokenType::Star),
            '!' => {
                let ty = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.token(ty)
            }
            '=' => {
                let ty = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.token(ty)
            }
            '<' => {
                let ty = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.token(ty)
            }
            '>' => {
                let ty = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.token(ty)
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        let _ = self.advance();
                    }
                    return SKIP_TOKEN;
                } else {
                    self.token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => return SKIP_TOKEN,
            '\n' => {
                self.line += 1;
                return SKIP_TOKEN;
            }
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => Err(Error::new(self.line, "Unexpected character.")),
        };
        Some(token).transpose()
    }

    fn token(&self, ty: TokenType) -> Result<Token> {
        let lexeme = self.current_lexeme().to_string();
        Ok(Token {
            ty,
            lexeme,
            line: self.line,
        })
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.source.chars().nth(self.current) {
            Some(c) if c == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn string(&mut self) -> Result<Token> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            let _ = self.advance();
        }

        if self.is_at_end() {
            return Err(Error::new(self.line, "Unterminated string."));
        }

        // The closing quote
        let _ = self.advance();

        let lexeme = self.current_lexeme();
        let literal = lexeme[1..(lexeme.len() - 1)].to_string();
        self.token(TokenType::String(literal))
    }

    fn number(&mut self) -> Result<Token> {
        while self.peek().is_ascii_digit() {
            let _ = self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            let _ = self.advance();

            while self.peek().is_ascii_digit() {
                let _ = self.advance();
            }
        }
        let number: f64 = self
            .current_lexeme()
            .parse()
            .map_err(|_| Error::new(self.line, "Invalid number literal."))?;
        self.token(TokenType::Number(number))
    }

    fn identifier(&mut self) -> Result<Token> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            let _ = self.advance();
        }
        let ty = KEYWORDS
            .get(&self.source[self.start..self.current])
            .cloned()
            .unwrap_or_else(|| TokenType::Identifier(self.current_lexeme().to_string()));
        self.token(ty)
    }

    fn current_lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }
}
