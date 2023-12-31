use crate::error::*;
use crate::scanner::{Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Token),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> Result<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let left = Box::new(expr);
            let operator = self
                .previous()
                .cloned()
                .expect("Lost equality operator token after matching");
            let right = Box::new(self.comparison()?);
            expr = Expr::Binary {
                left,
                operator,
                right,
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let left = Box::new(expr);
            let operator = self
                .previous()
                .cloned()
                .expect("Lost comparison operator token after matching");
            let right = Box::new(self.term()?);
            expr = Expr::Binary {
                left,
                operator,
                right,
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let left = Box::new(expr);
            let operator = self
                .previous()
                .cloned()
                .expect("Lost additive operator token after matching");
            let right = Box::new(self.factor()?);
            expr = Expr::Binary {
                left,
                operator,
                right,
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let left = Box::new(expr);
            let operator = self
                .previous()
                .cloned()
                .expect("Lost multiplicative operator token after matching");
            let right = Box::new(self.unary()?);
            expr = Expr::Binary {
                left,
                operator,
                right,
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self
                .previous()
                .cloned()
                .expect("Lost unary operator after matching");
            let right = Box::new(self.unary()?);
            Ok(Expr::Unary { operator, right })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.matches(&[
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number(0.),
            TokenType::String("".to_string()),
        ]) {
            Ok(Expr::Literal(
                self.previous()
                    .cloned()
                    .expect("Lost literal after matching"),
            ))
        } else if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            Err(Error::Syntax {
                line: self.peek().map(|t| t.line).unwrap_or_default(),
                message: "Expected expression.",
            })
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().unwrap().ty.matches(&TokenType::Semicolon) {
                return;
            }

            match self.peek().map(|t| &t.ty) {
                Some(TokenType::Class)
                | Some(TokenType::For)
                | Some(TokenType::Fun)
                | Some(TokenType::If)
                | Some(TokenType::Print)
                | Some(TokenType::Return)
                | Some(TokenType::Var)
                | Some(TokenType::While) => return,
                None => return,
                _ => {}
            }
            self.advance();
        }
    }

    fn consume(&mut self, ty: &TokenType, message: &'static str) -> Result<()> {
        if self.check(ty) {
            self.advance();
            Ok(())
        } else {
            Err(Error::Syntax {
                line: self.peek().map(|t| t.line).unwrap_or_default(),
                message,
            })
        }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        if types.iter().any(|t| self.check(t)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, ty: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().map(|t| t.ty.matches(ty)).unwrap_or_default()
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .map(|t| t.ty.matches(&TokenType::Eof))
            .unwrap_or(true)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
