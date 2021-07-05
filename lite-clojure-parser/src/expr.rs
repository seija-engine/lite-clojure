use super::{cexpr::Number, value::{Keyword, Symbol}};

#[derive(Debug,Clone)]
pub enum Expr {
    Nil,
    Fn(Vec<Symbol>,Vec<Expr>),
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

impl Expr {
    pub fn case_vector(self) -> Option<Vec<Expr>> {
        match self {
            Expr::Vector(lst) => Some(lst),
            _ => None
        }
    }

    pub fn case_sym(self) -> Option<Symbol> {
        match self {
            Expr::Symbol(sym) => Some(sym),
            _ => None
        }
    }

}