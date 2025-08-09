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

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expr, Visitor},
        ast_printer::AstPrinter,
        token::{Token, Value},
        token_type::TokenType,
    };

    #[test]
    fn test_example() {
        let e = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".into(), Value::None, 1),
                right: Box::new(Expr::Literal(123_f64.into())),
            }),
            operator: Token::new(TokenType::Star, "*".into(), Value::None, 1),
            right: Box::new(Expr::Grouping(Box::new(Expr::Literal((45.67).into())))),
        };
        assert_eq!("(* (- 123) (group 45.67))", AstPrinter {}.visit_expr(&e));
    }
}
