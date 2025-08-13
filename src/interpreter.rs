#![allow(dead_code)]

use std::fmt::Display;

use thiserror::Error;

use crate::{
    ast::{Expr, Visitor},
    lox::Lox,
    token::{Token, Value},
    token_type::TokenType,
};

#[derive(Error, Debug)]
pub struct RuntimeError {
    token: Token,
    message: std::string::String,
}

impl RuntimeError {
    fn new(token: Token, message: impl Into<String>) -> Self {
        Self {
            token: token.clone(),
            message: message.into(),
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0} [line {1}]", self.message, self.token.line)
    }
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    lox: &'a mut Lox,
}

impl<'a> Interpreter<'a> {
    pub fn new(lox: &'a mut Lox) -> Self {
        Self { lox }
    }

    pub fn interpret(&mut self, expression: &Expr) {
        match self.evaluate(expression) {
            Ok(value) => println!("{}", value.to_string()),
            Err(error) => self.lox.runtime_error(error),
        }
    }

    pub fn evaluate(&self, expression: &Expr) -> Result<Value, RuntimeError> {
        self.visit_expr(expression)
    }
}

impl<'a> Visitor<Result<Value, RuntimeError>> for Interpreter<'a> {
    fn visit_expr(&self, expr: &crate::ast::Expr) -> Result<Value, RuntimeError> {
        match expr {
            crate::ast::Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expr(left)?;
                let right = self.visit_expr(right)?;
                match operator.token_type {
                    TokenType::Minus => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left - right))
                        }
                        _ => todo!(),
                    },
                    TokenType::Slash => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left / right))
                        }
                        _ => todo!(),
                    },
                    TokenType::Star => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left * right))
                        }
                        _ => todo!(),
                    },
                    _ => panic!("Unexpected binary operator!"),
                }
            }
            Expr::Grouping(expr) => self.visit_expr(expr),
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Unary { operator, right } => {
                let right = self.visit_expr(right)?;
                match operator.token_type {
                    TokenType::Bang => Ok(Value::Bool(is_truthy(right))),
                    TokenType::Minus => match right {
                        Value::Number(value) => Ok(Value::Number(-value)),
                        _ => panic!("Unary expression was not a number!"),
                    },
                    _ => panic!("Unexpected unary operator!"), // Unexpected unary operator.
                }
            }
        }
    }
}

fn is_truthy(value: Value) -> bool {
    match value {
        Value::Number(_) => true,
        Value::String(_) => true,
        Value::Bool(value) => value,
        Value::None => false,
    }
}
