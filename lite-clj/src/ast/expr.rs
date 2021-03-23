use super::value::Symbol;

pub enum Expr {
    Nil,
    Fn,
    Def(Option<String>,Symbol,Option<Box<Expr>>)
}