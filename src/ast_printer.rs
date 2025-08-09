use crate::expr::{self, *};

struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: &[Expr]) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(name);
        for expr in exprs.iter() {
            result.push_str(" ");
            result.push_str(expr.accept(&self));
        }
        result.push_str(")");
        result
    }
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        let exprs: Vec<Expr> = vec![Expr::Binary(*expr)];
        parenthesize(&expr.operator.lexeme, &exprs)
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        todo!()
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        todo!()
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        todo!()
    }
}
