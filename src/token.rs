use std::fmt::{Debug, Display};

use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::String(value) => f.write_str(value),
            Self::None => f.write_str("nil")
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<Option<String>> for Value {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Value::String(s),
            None => Value::None,
        }
    }
}

impl From<Option<f64>> for Value {
    fn from(value: Option<f64>) -> Self {
        match value {
            Some(f) => Value::Number(f),
            None => Value::None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Value,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Value, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
