use std::collections::HashMap;
use std::string::String;
use std::sync::OnceLock;

use crate::lox::Lox;
use crate::token::{Token, Value};
use crate::token_type::TokenType::{self, *};

fn keywords() -> &'static HashMap<String, TokenType> {
    static INSTANCE: OnceLock<HashMap<String, TokenType>> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let mut keywords = HashMap::new();
        keywords.insert("and".into(), And);
        keywords.insert("class".into(), Class);
        keywords.insert("else".into(), Else);
        keywords.insert("false".into(), False);
        keywords.insert("for".into(), For);
        keywords.insert("fun".into(), Fun);
        keywords.insert("if".into(), If);
        keywords.insert("nil".into(), Nil);
        keywords.insert("or".into(), Or);
        keywords.insert("print".into(), Print);
        keywords.insert("return".into(), Return);
        keywords.insert("super".into(), Super);
        keywords.insert("this".into(), This);
        keywords.insert("true".into(), True);
        keywords.insert("var".into(), Var);
        keywords.insert("while".into(), While);

        keywords
    })
}

pub fn scan_tokens(lox: &mut Lox, source: &str) -> Vec<Token> {
    Scanner::new(source).scan_tokens(lox)
}

#[derive(Default, Debug)]
struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
            ..Default::default()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token_literal(&mut self, t: TokenType, literal: impl Into<Value>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(t, text, literal.into(), self.line));
    }

    fn add_token(&mut self, t: TokenType) {
        self.add_token_literal(t, Value::None);
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn scan_token(&mut self, lox: &Lox) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = if self.char_match('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.char_match('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.char_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(lox),
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    lox.error(self.line, "Unexpected character.");
                }
            }
        };
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let tt = keywords().get(&text).unwrap_or(&Identifier);
        self.add_token(*tt);
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance(); // consume the '.'

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .expect("Numeric String has already been scanned.");
        self.add_token_literal(Number, value);
    }

    fn string(&mut self, lox: &Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            lox.error(self.line, "Unterminated string.");
            return;
        }
        self.advance(); // the closing '"'
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(String, value);
    }

    fn scan_tokens(mut self, lox: &Lox) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(lox);
        }

        self.tokens
            .push(Token::new(EOF, "".into(), Value::None, self.line));
        self.tokens
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lox::Lox;

    #[test]
    fn test_identifier() {
        let t1 = Token::new(Or, "or".into(), Value::None, 1);
        let t2 = Token::new(Identifier, "thing".into(), Value::None, 1);
        let tokens = scan_tokens(&mut Lox::new(), "or thing");
        assert_eq!(tokens[0], t1);
        assert_eq!(tokens[1], t2);
    }
}
