use crate::token::Token;
use crate::token_type::TokenType::*;

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Default::default(),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while (!self.is_at_end()) {
            self.start = self.current;
        }

        self.tokens
            .push(Token::new(EOF, "".into(), None, self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
