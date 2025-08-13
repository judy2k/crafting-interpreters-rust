use crate::token::{Token, Value};

pub trait ExprVisitor<R> {
    fn visit_expr(&self, expr: &Expr) -> R;
}

#[derive(Debug, PartialEq)]
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

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    pub fn grouping(expr: Expr) -> Self {
        Self::Grouping(Box::new(expr))
    }
    pub fn literal(value: impl Into<Value>) -> Self {
        Self::Literal(value.into())
    }
    pub fn unary(operator: Token, right: Expr) -> Self {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }
}

pub trait StmtVisitor<R> {
    fn visit_stmt(&self, expr: &Expr) -> R;
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

impl Stmt {
    pub fn expression(expression: Expr) -> Self {
        Self::Expression(expression)
    }
    pub fn print(expression: Expr) -> Self {
        Self::Print(expression)
    }
}
