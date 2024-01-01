use crate::error::{Error, Result};
use crate::scanner::TokenType;
use crate::syntax::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s:?}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl std::ops::Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.is_truthy().not().into()
    }
}

impl Value {
    fn into_double(self, line: usize) -> Result<f64> {
        if let Self::Number(num) = self {
            Ok(num)
        } else {
            Err(Error::TypeError {
                line,
                message: "Expected number",
            })
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::String(_) | Value::Number(_) => true,
            Value::Boolean(b) => *b,
            Value::Nil => false,
        }
    }

    #[allow(dead_code)]
    fn kind(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::Number(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Nil => "nil",
        }
    }
}

pub fn interpret(expr: Expr) {
    match evaluate(expr) {
        Ok(v) => println!("{v}"),
        Err(e) => eprintln!("{e}"),
    }
}

pub fn evaluate(expr: Expr) -> Result<Value> {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;
            match operator.ty {
                TokenType::Minus => {
                    let left = left.into_double(operator.line)?;
                    let right = right.into_double(operator.line)?;
                    Ok((left - right).into())
                }
                TokenType::Slash => {
                    let left = left.into_double(operator.line)?;
                    let right = right.into_double(operator.line)?;
                    Ok((left / right).into())
                }
                TokenType::Star => {
                    let left = left.into_double(operator.line)?;
                    let right = right.into_double(operator.line)?;
                    Ok((left * right).into())
                }
                TokenType::Plus => match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok((l + r).into()),
                    (Value::String(l), Value::String(r)) => Ok((l + &r).into()),
                    _ => Err(Error::TypeError {
                        line: operator.line,
                        message: "Invalid operand types for '+'",
                    }),
                },
                TokenType::Greater => {
                    let left = left.into_double(operator.line)?;
                    let right = right.into_double(operator.line)?;
                    Ok((left > right).into())
                }
                TokenType::GreaterEqual => {
                    let left = left.into_double(operator.line)?;
                    let right = right.into_double(operator.line)?;
                    Ok((left >= right).into())
                }
                TokenType::Less => {
                    let left = left.into_double(operator.line)?;
                    let right = right.into_double(operator.line)?;
                    Ok((left < right).into())
                }
                TokenType::LessEqual => {
                    let left = left.into_double(operator.line)?;
                    let right = right.into_double(operator.line)?;
                    Ok((left <= right).into())
                }
                TokenType::BangEqual => Ok((left != right).into()),
                TokenType::EqualEqual => Ok((left == right).into()),
                _ => panic!("Invalid binary operator"),
            }
        }
        Expr::Grouping(e) => evaluate(*e),
        Expr::Literal(token) => match token.ty {
            TokenType::Number(num) => Ok(num.into()),
            TokenType::String(s) => Ok(s.into()),
            TokenType::False => Ok(false.into()),
            TokenType::True => Ok(true.into()),
            TokenType::Nil => Ok(Value::Nil),
            _ => panic!("Invalid literal value"),
        },
        Expr::Unary { operator, right } => {
            let right = evaluate(*right)?;
            match operator.ty {
                TokenType::Minus => {
                    let right = right.into_double(operator.line)?;
                    Ok((-right).into())
                }
                TokenType::Bang => Ok(!right),
                _ => panic!("Invalid unary operator"),
            }
        }
    }
}
