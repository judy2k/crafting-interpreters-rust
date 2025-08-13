use std::{
    fs::read_to_string,
    io::{self, Write},
    path::Path,
};

use thiserror::Error;

use crate::{
    ast::Expr, ast_printer::AstPrinter, interpreter::RuntimeError, parser::parse, scanner::scan_tokens, token::Token, token_type::TokenType
};

#[derive(Debug, Error)]
pub enum LoxError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}

#[derive(Default, Debug)]
pub struct Lox {
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run_file(&mut self, path: &Path) -> Result<(), LoxError> {
        let code = read_to_string(path)?;
        self.run(&code);

        if self.had_error {
            std::process::exit(65);
        }
        
        if self.had_runtime_error { 
            std::process::exit(70);
        }


        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), LoxError> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("> ");
            stdout.flush()?;
            buffer.clear();
            stdin.read_line(&mut buffer)?;
            // TODO: Handle end of stream.
            self.run(&buffer);
        }
    }

    fn run(&mut self, code: &str) {
        let expression = self.parse_code(code);
        if self.had_error {
            return;
        }

        println!(
            "{}",
            AstPrinter::new().print(&expression.expect("Expression unexpectedly None!"))
        );
    }

    pub(crate) fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message)
    }

    fn report(&mut self, line: usize, loc: &str, message: &str) {
        eprintln!("[line {line}] Error {loc} : {message}");
        self.had_error = true;
    }

    pub(crate) fn runtime_error(&mut self, error: RuntimeError) {
        println!("{}", error);
        self.had_runtime_error = true;
    }

    pub(crate) fn parse_error(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    fn scan_tokens(&mut self, code: &str) -> Vec<Token> {
        scan_tokens(self, code)
    }

    pub fn parse_code(&mut self, code: &str) -> Option<Expr> {
        let tokens = self.scan_tokens(code);
        parse(self, tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lox_default() {
        let l: Lox = Default::default();
        assert!(!l.had_error, "Lox should be created with no errors.")
    }
}
