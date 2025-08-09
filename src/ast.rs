use crate::token::{Token, Value};

pub trait Visitor<R> {
    fn visit_expr(&self, expr: &Expr) -> R;
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Value),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
