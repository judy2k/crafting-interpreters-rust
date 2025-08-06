use std::cell::RefCell;
use std::string::String;

use crate::lox::Lox;
use crate::token::{Token, Value};
use crate::token_type::TokenType::{self, *};


pub fn scan_tokens(lox: &mut Lox, source: &str) -> Vec<Token> {
    let source: Vec<char> = source.chars().collect();
    let start: RefCell<usize> = RefCell::new(0);
    let current: RefCell<usize> = RefCell::new(0);
    let mut line =  1;
    let tokens: RefCell<Vec<Token>> = RefCell::new(vec![]);

    let is_at_end = || current.borrow().ge(&source.len());
    let advance = || {
        let mut current = current.borrow_mut();
        let result = source[*current];
        *current += 1;
        result
    };
    let add_token_literal = |t: TokenType, literal: Value| {
        let text: String = source[*start.borrow()..*current.borrow()].iter().collect();
        tokens.borrow_mut().push(Token::new(t, text, literal, line));
    };
    let add_token = |t: TokenType| add_token_literal(t, Value::None);
    
    let mut scan_token = || {
        let c = advance();
        match c {
            '(' => add_token(LeftParen),
            ')' => add_token(RightParen),
            '{' => add_token(LeftBrace),
            '}' => add_token(RightBrace),
            ',' => add_token(Comma),
            '.' => add_token(Dot),
            '-' => add_token(Minus),
            '+' => add_token(Plus),
            ';' => add_token(Semicolon),
            '*' => add_token(Star),
            _ => lox.error(line, "Unexpected character."),
        };
    };
    
    while !is_at_end() {
        *start.borrow_mut() = *current.borrow();
        scan_token();
    }

    tokens.borrow_mut()
        .push(Token::new(EOF, "".into(), Value::None, line));
    tokens.take()
}



