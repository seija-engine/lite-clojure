use super::{cexpr::Number, value::{Keyword, Symbol}};

#[derive(Debug)]
pub enum Expr {
    Nil,
    Fn,
    Number(Number),
    Boolean(bool),
    Symbol(Symbol),
    Keyword(Keyword),
    String(String),

    Def(Option<String>,Symbol,Option<Box<Expr>>),
    Let(Vec<Expr>,Box<Expr>,bool),
    Body(Vec<Expr>),
    Invoke(Vec<Expr>),
    If(Box<Expr>,Box<Expr>,Box<Expr>),

    Map(Vec<Expr>),
    Vector(Vec<Expr>)
}