

use crate::{
    ast::{Expr, Visitor},
    ast_printer::AstPrinter,
    token::{Token, Value},
};

mod ast;
mod ast_printer;
mod lox;
mod scanner;
mod token;
mod token_type;

// fn main() -> eyre::Result<()> {
//     let args: Vec<String> = env::args().collect();
//     let mut lox = lox::Lox::new();
//     println!("Args: {args:?}");

//     if args.len() > 2 {
//         return Err(eyre!("Usage lox [script]"));
//     } else if args.len() == 2 {
//         lox.run_file(Path::new(&args[1]))?;
//     } else {
//         lox.run_prompt()?;
//     }

//     Ok(())
// }

fn main() -> eyre::Result<()> {
    let e = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(token_type::TokenType::Minus, "-".into(), Value::None, 1),
            right: Box::new(Expr::Literal(123_f64.into())),
        }),
        operator: Token::new(token_type::TokenType::Star, "*".into(), Value::None, 1),
        right: Box::new(Expr::Grouping(Box::new(Expr::Literal((45.67).into())))),
    };
    println!("{}", AstPrinter {}.visit_expr(&e));
    Ok(())
}
