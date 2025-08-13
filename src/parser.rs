use thiserror::Error;

use crate::ast::Stmt;
use crate::lox::LoxReporter;
use crate::token::Value;
use crate::token_type::TokenType::{self, *};
use crate::{ast::Expr, token::Token};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("An error occured with parsing.")]
    Error,
}

type ExprResult = Result<Expr, ParseError>;
type StmtResult = Result<Stmt, ParseError>;

#[derive(Debug)]
pub struct Parser<'a> {
    reporter: &'a mut LoxReporter,
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(reporter: &'a mut LoxReporter, tokens: &'a [Token]) -> Self {
        Self {
            reporter,
            tokens,
            current: 0,
        }
    }

    fn parse(reporter: &'a mut LoxReporter, tokens: &'a [Token]) -> Result<Vec<Stmt>, ParseError> {
        let mut parser = Self::new(reporter, tokens);
        let mut statements: Vec<Stmt> = vec![];
        while !parser.is_at_end() {
            statements.push(parser.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> StmtResult {
        todo!()
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
        self.reporter.parse_error(token, message);
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

pub fn parse(reporter: &mut LoxReporter, tokens: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
    Parser::parse(reporter, &tokens)
}

#[cfg(test)]
mod tests {
    use crate::ast::Expr;
    use crate::lox::LoxReporter;
    use crate::parser::ExprResult;
    use crate::scanner::scan_tokens;
    use crate::token::{Token, Value};

    use super::Parser;

    fn parse_expression(expression: &str) -> (ExprResult, bool) {
        let mut reporter: LoxReporter = Default::default();
        let tokens = scan_tokens(&mut reporter, expression);
        let expr = Parser::new(&mut reporter, &tokens).expression();

        (expr, reporter.had_error)
    }

    #[test]
    fn test_addition() {
        let (expr, had_error) = parse_expression("1 + 2");

        assert!(!had_error);
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

        assert_eq!(expr.unwrap(), expr2);
    }

    #[test]
    fn test_multiplication() {
        let (expr, had_error) = parse_expression("1 * 2");
        assert!(!had_error);
        let expr2 = Expr::binary(
            Expr::Literal(1.into()),
            Token::new(
                crate::token_type::TokenType::Star,
                "*".to_string(),
                Value::None,
                1,
            ),
            Expr::Literal(2.into()),
        );

        assert_eq!(expr.unwrap(), expr2);
    }

    #[test]
    fn test_division() {
        let (expr, had_error) = parse_expression("1 / 2");
        assert!(!had_error);
        let expr2 = Expr::binary(
            Expr::Literal(1.into()),
            Token::new(
                crate::token_type::TokenType::Slash,
                "/".to_string(),
                Value::None,
                1,
            ),
            Expr::Literal(2.into()),
        );

        assert_eq!(expr.unwrap(), expr2);
    }

    #[test]
    fn test_group() {
        let (expr, had_error) = parse_expression("(1)");
        assert!(!had_error);
        assert_eq!(expr.unwrap(), Expr::grouping(Expr::Literal(1.into())));
    }

    #[test]
    fn test_unmatched_paren() {
        for s in ["1 + (2", "(", "(1"] {
            let (expr, had_error) = parse_expression(s);
            assert!(had_error);
            assert!(expr.is_err());
        }
    }

    #[test]
    fn test_bang() {
        let (expr, had_error) = parse_expression("!true");
        assert!(!had_error);
        let expr2 = Expr::unary(
            Token::new(
                crate::token_type::TokenType::Bang,
                "!".into(),
                Value::None,
                1,
            ),
            Expr::literal(true),
        );

        assert_eq!(expr.unwrap(), expr2);
    }

    #[test]
    fn test_precedence() {
        let (expr, had_error) = parse_expression("1 * 2 + 3 / 4");
        assert!(!had_error);

        let left = parse_expression("1 * 2").0.unwrap();
        let right = parse_expression("3 / 4").0.unwrap();
        let expr2 = Expr::binary(
            left,
            Token::new(
                crate::token_type::TokenType::Plus,
                "+".to_string(),
                Value::None,
                1,
            ),
            right,
        );

        assert_eq!(expr.unwrap(), expr2);
    }
}
