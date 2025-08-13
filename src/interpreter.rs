#![allow(dead_code)]

use std::fmt::Display;

use thiserror::Error;

use crate::{
    ast::{Expr, Stmt, ExprVisitor},
    lox::LoxReporter,
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

#[derive(Debug, Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn execute(&mut self, reporter: &mut LoxReporter, statements: &Vec<Stmt>) {
        // match self.evaluate(expression) {
        //     Ok(value) => println!("{value}"),
        //     Err(error) => reporter.runtime_error(error),
        // }
        unimplemented!()
    }

    pub fn interpret(&mut self, reporter: &mut LoxReporter, expression: &Expr) {
        match self.evaluate(expression) {
            Ok(value) => println!("{value}"),
            Err(error) => reporter.runtime_error(error),
        }
    }

    pub fn evaluate(&self, expression: &Expr) -> Result<Value, RuntimeError> {
        self.visit_expr(expression)
    }
}

#[inline]
fn number_operands_error(operator: &Token) -> RuntimeError {
    RuntimeError::new(operator.clone(), "Operands must be numbers.")
}

impl ExprVisitor<Result<Value, RuntimeError>> for Interpreter {
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
                        _ => Err(number_operands_error(operator)),
                    },
                    TokenType::Plus => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left + right))
                        }
                        (Value::String(left), Value::String(right)) => {
                            let mut result = left.to_string();
                            result.push_str(&right);
                            Ok(Value::String(result))
                        }
                        _ => Err(number_operands_error(operator)),
                    },
                    TokenType::Slash => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left / right))
                        }
                        _ => Err(number_operands_error(operator)),
                    },
                    TokenType::Star => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left * right))
                        }
                        _ => Err(number_operands_error(operator)),
                    },
                    TokenType::Greater => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Bool(left > right))
                        }
                        _ => Err(number_operands_error(operator)),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Bool(left >= right))
                        }
                        _ => Err(number_operands_error(operator)),
                    },
                    TokenType::Less => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Bool(left < right))
                        }
                        _ => Err(number_operands_error(operator)),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Bool(left <= right))
                        }
                        _ => Err(number_operands_error(operator)),
                    },
                    TokenType::BangEqual => Ok(Value::Bool(left != right)),
                    TokenType::EqualEqual => Ok(Value::Bool(left == right)),
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
