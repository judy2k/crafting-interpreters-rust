use crate::ast::{self, *};

pub(crate) struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in exprs.iter() {
            result.push(' ');
            result.push_str(&self.visit_expr(expr));
        }
        result.push(')');
        result
    }
}

impl ast::Visitor<String> for AstPrinter {
    fn visit_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(&operator.lexeme, &[left.as_ref(), right.as_ref()]),
            Expr::Grouping(expr) => self.parenthesize("group", &[expr.as_ref()]),
            Expr::Literal(value) => value.to_string(),
            Expr::Unary { operator, right } => {
                self.parenthesize(&operator.lexeme, &[right.as_ref()])
            }
        }
    }
}
