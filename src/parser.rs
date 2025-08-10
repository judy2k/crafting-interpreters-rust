use thiserror::Error;

use crate::lox::Lox;
use crate::token::Value;
use crate::token_type::TokenType::{self, *};
use crate::{ast::Expr, token::Token};

#[derive(Error, Debug)]
enum ParseError {
    #[error("An error occured with parsing.")]
    Error,
}

type ExprResult = Result<Expr, ParseError>;

#[derive(Debug)]
pub struct Parser<'a> {
    lox: &'a mut Lox,
    tokens: Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(lox: &'a mut Lox, tokens: Vec<Token>) -> Self {
        Self {
            lox,
            tokens,
            current: 0,
        }
    }

    fn parse(lox: &'a mut Lox, tokens: Vec<Token>) -> Option<Expr> {
        Self::new(lox, tokens).expression().ok()
    }

    fn expression(&mut self) -> ExprResult {
        self.equality()
    }

    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;

        while self.token_match(&[BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ExprResult {
        let mut expr = self.term()?;

        while self.token_match(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> ExprResult {
        let mut expr = self.factor()?;
        while self.token_match(&[Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ExprResult {
        let mut expr = self.unary()?;
        while self.token_match(&[Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ExprResult {
        if self.token_match(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> ExprResult {
        if self.token_match(&[False]) {
            return Ok(Expr::literal(false));
        }
        if self.token_match(&[True]) {
            return Ok(Expr::literal(true));
        }
        if self.token_match(&[Nil]) {
            return Ok(Expr::literal(Value::None));
        }

        if self.token_match(&[Number, String]) {
            return Ok(Expr::literal(self.previous().literal.clone()));
        }

        if self.token_match(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::grouping(expr));
        }

        Err(self.error(&self.peek().clone(), "Expect expression."))
    }

    // -------------------------------------------------------------------------

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        }
        Err(self.error(&self.peek().clone(), message))
    }

    fn token_match(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&mut self, token: &Token, message: &str) -> ParseError {
        self.lox.parse_error(token, message);
        ParseError::Error
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }
            match self.peek().token_type {
                Class | For | Fun | If | Print | Return | Var | While => return,
                _ => (),
            }
            self.advance();
        }
    }
}

pub fn parse(lox: &mut Lox, tokens: Vec<Token>) -> Option<Expr> {
    Parser::parse(lox, tokens)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Expr,
        lox::Lox,
        token::{Token, Value},
    };

    #[test]
    fn test_expressions() {
        let mut lox = Lox::new();
        let expr = lox.parse_code("1 + 2").unwrap();
        assert!(!lox.had_error);
        let expr2 = Expr::binary(
            Expr::Literal(1.into()),
            Token::new(
                crate::token_type::TokenType::Plus,
                "+".to_string(),
                Value::None,
                1,
            ),
            Expr::Literal(2.into()),
        );

        assert_eq!(expr, expr2);
    }

    #[test]
    fn test_unmatched_paren() {
        let mut lox = Lox::new();
        let expr = lox.parse_code("1 + (2");
        assert!(lox.had_error);
        assert!(expr.is_none());
    }
}
