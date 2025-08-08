use std::{env, path::Path};

use eyre::eyre;

mod ast_gen;
mod expr;
mod lox;
mod scanner;
mod token;
mod token_type;

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();
    println!("Args: {args:?}");

    if args.len() > 2 {
        return Err(eyre!("Usage lox [script]"));
    } else if args.len() == 2 {
        lox.run_file(Path::new(&args[1]))?;
    } else {
        lox.run_prompt()?;
    }

    Ok(())
}
