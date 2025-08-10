use std::{
    fs::read_to_string,
    io::{self, Error, Write},
    path::Path,
};

use crate::{
    ast::Expr, ast_printer::AstPrinter, parser::parse, scanner::scan_tokens, token::Token,
    token_type::TokenType,
};

#[derive(Default, Debug)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run_file(&mut self, path: &Path) -> Result<(), Error> {
        let code = read_to_string(path)?;
        self.run(&code);

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), Error> {
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
        let tokens = scan_tokens(self, code);
        let expression = parse(self, tokens);
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

    pub(crate) fn parse_error(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    pub fn scan_tokens(&mut self, code: &str) -> Vec<Token> {
        scan_tokens(self, code)
    }

    pub fn parse_code(&mut self, code: &str) -> Option<Expr> {
        let tokens = scan_tokens(self, code);
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
