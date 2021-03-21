use super::{errors::{ASTError, CSTError}, expr::{CExpr, Expr}};

pub struct ParseAST {
    exprs:Vec<CExpr>
}

impl ParseAST {
    pub fn parse(&mut self,cexpr:&CExpr) -> Result<Expr,ASTError> {
        todo!()
    }

}