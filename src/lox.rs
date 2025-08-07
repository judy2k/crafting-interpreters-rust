use std::{
    fs::read_to_string,
    io::{self, Error, Write},
    path::Path,
};

#[derive(Default, Debug)]
pub struct Lox {
    has_error: bool,
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
            stdin.read_line(&mut buffer)?;
            // TODO: Handle end of stream.
            self.run(&buffer);
        }
    }

    fn run(&mut self, code: &str) {}

    pub fn error(&self, line: usize, message: &str) {
        self.report(line, "", message)
    }

    fn report(&self, line: usize, loc: &str, message: &str) {
        eprintln!("[line {line}] Error {loc} : {message}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lox_default() {
        let l: Lox = Default::default();
        assert!(!l.has_error, "Lox should be created with no errors.")
    }
}
