use crate::ast_gen::ast_node;
use paste::paste;

use crate::token::{Token, Value};

ast_node!(
Expr,
Binary   -> left: Box<Expr>, operator: Token, right: Box<Expr>;
Grouping -> expression: Box<Expr>;
Literal -> value: Value;
Unary -> operator: Token, right: Box<Expr>;
 );
