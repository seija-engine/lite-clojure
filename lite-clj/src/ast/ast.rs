use super::{cexpr::{CExpr}, errors::{ASTError}, expr::Expr, meta::MetaTable};
pub struct ASTModule {
    pub file_name:String,
    pub exprs:Vec<Expr>
}

pub struct TranslateToAST {
    file_name:String,
    cexprs:Vec<CExpr>,
    meta_table:MetaTable<CExpr>,

    exprs:Vec<Expr>,
}

impl TranslateToAST {
    pub fn new(file_name:String,cexprs:Vec<CExpr>,meta_table:MetaTable<CExpr>) -> Self {
       TranslateToAST {file_name,cexprs,meta_table,exprs:vec![]}
    }
    


    pub fn translate(mut self) -> ASTModule {
        let drain_cexprs:Vec<CExpr> = self.cexprs.drain(0..).collect();
        for cexpr in drain_cexprs {
            self.translate_cexpr(cexpr)
        }

        ASTModule {
            file_name : self.file_name,
            exprs:self.exprs
        }
    }

    fn translate_cexpr(&mut self,cexpr:CExpr) {
        //TODO 宏展开
        let expr = self.analyze(cexpr).unwrap();
        self.exprs.push(expr);
    }

    fn analyze(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
       if cexpr.is_iseq() {
           self.analyze_seq(cexpr)?;
       }
       Ok(Expr::Nil)
    }

    fn analyze_seq(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
        let mop = cexpr.seq_first();
        if mop.is_none() {
            return Err(ASTError::ErrSeq);    
        }
        if let Some(sym) = mop.unwrap().cast_sym() {
            if  sym.sym_ns().is_none() {
                match sym.sym_name() {
                    "fn*" => {},
                    "def" => {
                        return self.parse_def_expr(cexpr);
                    },
                    _ => ()
                }
            }
        }
        Err(ASTError::ErrSeq)
    }

    fn parse_def_expr(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
        // (def x) or (def x initexpr) or (def x "docstring" initexpr)
        let mut lst = cexpr.take_seq_list().unwrap();
        let mut doc_string:Option<String> = None;
        if lst.len() == 4 && lst[2].is_string() {
            doc_string = Some(lst.remove(2).cast_string().unwrap());
        }
        if lst.len() > 3 || lst.len() < 2 {
            return Err(ASTError::ArgErrorDef);
        }

        lst.remove(0);
        let sym =  lst.remove(0).cast_symbol().unwrap();
        let mut init_expr:Option<Box<Expr>> = None;
        if lst.len() > 0 {
           init_expr = Some(Box::new(self.analyze(lst.remove(0))?));
        }
        Ok(Expr::Def(doc_string,sym,init_expr))
    }
}


#[test]
fn test_trans() {
   use super::cst::ParseCST;
   let file_name = "tests/clj/test.clj";
   let code_string = std::fs::read_to_string(file_name).unwrap();
   let mut parser = ParseCST::new(&code_string);
   let cexprs = parser.parse_exprs().unwrap();
   let meta_table = parser.take();

   let mut trans = TranslateToAST::new(file_name.to_string(), cexprs, meta_table);
   trans.translate();
}