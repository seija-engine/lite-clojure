pub enum Expr {
    Lit(LiteralExpr),
    Eof
}

pub enum LiteralExpr {
    Nil,
    Boolean(bool),
    Keyword(String),
    String(String),
    Number(Number)
}

pub enum Number {
    Int(i64),
    Float(f64)
}