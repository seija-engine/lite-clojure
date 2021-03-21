use super::{cexpr::{CExpr}, errors::{ASTError, CSTError}, expr::Expr};

pub struct TranslateToAST {
    exprs:Vec<CExpr>
}

impl TranslateToAST {
    pub fn translate(&mut self,cexpr:&CExpr) -> Result<Expr,ASTError> {

        todo!()
    }

    fn analyze(&mut self,cexpr:&CExpr) -> Result<Expr,ASTError> {
        todo!()
    }

}