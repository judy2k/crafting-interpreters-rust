enum Token {}

enum Expr {
    Assign(Assign),
    Binary(Binary),
}

trait Visitor<R> {
    fn visitAssignExpr(&self, expr: &Assign) -> R;
    fn visitBinaryExpr(&self, expr: &Binary) -> R;
}

struct Assign {
    name: Token,
    value: Box<Expr>,
}

impl Assign {
    fn new() {}

    fn accept<R>(&mut self, visitor: impl Visitor<R>) -> R {
        return visitor.visitAssignExpr(self);
    }
}

struct Binary {}

impl Binary {
    fn accept<R>(&mut self, visitor: impl Visitor<R>) -> R {
        return visitor.visitBinaryExpr(self);
    }
}

struct Printer();

impl Printer {
    fn print(expr: &Expr) {}
}

impl Visitor<String> for Printer {
    fn visitAssignExpr(&self, expr: &Assign) -> String {
        return "It's an assign expression!".into();
    }
    fn visitBinaryExpr(&self, expr: &Binary) -> String {
        return "It's a binary expression!".into();
    }
}

fn main() {
    println!("Hello, world!");
}
