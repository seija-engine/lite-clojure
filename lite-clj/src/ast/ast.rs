use super::{cexpr::{CExpr}, errors::{ASTError}, expr::Expr, meta::MetaTable, value::Symbol};
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
        dbg!(&expr);
        self.exprs.push(expr);
    }

    fn analyze(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
       match cexpr {
           CExpr::Nil => Ok(Expr::Nil),
           CExpr::Boolean(b) => Ok(Expr::Boolean(b)),
           CExpr::Symbol(sym) => self.analyze_sym(sym),
           CExpr::Number(_raw,num)  => Ok(Expr::Number(num)),
           CExpr::Keyword(key) => Ok(Expr::Keyword(key)),
           CExpr::String(str) => Ok(Expr::String(str)),
           _ => {
            if cexpr.is_iseq() {
                self.analyze_seq(cexpr)
            } else {
                todo!()
            }
           }
       }
    }

    fn analyze_sym(&mut self,sym:Symbol) -> Result<Expr,ASTError> {
        //TODO
        Ok(Expr::Symbol(sym))
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
                    "def" =>  return self.parse_def_expr(cexpr),
                    "loop*" => return self.parse_let_expr(cexpr,true),
                    "let*" =>  return self.parse_let_expr(cexpr, false),
                    "if" => return self.parse_if_expr(cexpr),
                    "case*" => return self.parse_case_expr(),
                    _ => return self.parse_invoke(cexpr)
                }
            }
        }
        
        Err(ASTError::ErrSeq)
    }

    fn parse_case_expr(&mut self) -> Result<Expr,ASTError> {
        todo!()
    }

    fn parse_if_expr(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
        // (if test then) or (if test then else)
        let mut lst = cexpr.take_list().unwrap();
        if lst.len() > 4 || lst.len() < 3 {
            return Err(ASTError::ErrIf);
        }
        lst.remove(0);
        let test_expr = self.analyze(lst.remove(0)).unwrap();
        let then_expr = self.analyze(lst.remove(0)).unwrap();
        let else_expr = if lst.len() > 0 {
            self.analyze(lst.remove(0)).unwrap()
        } else {Expr::Nil };
        Ok(Expr::If(Box::new(test_expr),Box::new(then_expr),Box::new(else_expr)))
    }

    fn parse_def_expr(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
        // (def x) or (def x initexpr) or (def x "docstring" initexpr)
        let mut lst = cexpr.take_list().unwrap();
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

    fn parse_let_expr(&mut self,cexpr:CExpr,is_loop:bool) -> Result<Expr,ASTError> {
        //(let  [var1 val1 var2 val2 ... ] body ... )
        //(loop [var1 val1 var2 val2 ... ] body ... )
        let mut lst = cexpr.take_list().unwrap();
        if lst.len() < 2 {
            return Err(ASTError::ErrLet);
        }
        lst.remove(0);
        if !lst[0].is_vec() {
            return Err(ASTError::BadBindingForm);
        }
        let mut bindings = lst.remove(0).take_list().unwrap();
        if (bindings.len() % 2) != 0 {
            return Err(ASTError::ErrLet);
        }

        let mut bind_vecs:Vec<Expr> = vec![];
        for _idx in 0..bindings.len() / 2 {
            let cexpr = bindings.remove(0);
            let sym_expr = self.analyze(cexpr)?;
            let val_cexpr = bindings.remove(0);
            let val_expr = self.analyze(val_cexpr)?;
            bind_vecs.push(sym_expr);
            bind_vecs.push(val_expr);
        }
        let body_expr = self.parse_do_expr_(lst)?;
        Ok(Expr::Let(bind_vecs,Box::new(body_expr),is_loop))
    }

    fn parse_do_expr_(&mut self,cexprs:Vec<CExpr>) -> Result<Expr,ASTError> {
        let mut exprs:Vec<Expr> = vec![];
        for cexpr in cexprs {
            exprs.push(self.analyze(cexpr)?);
        }
        Ok(Expr::Body(exprs))
    }

    fn parse_invoke(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
        let mut exprs:Vec<Expr> = vec![];
        for cexpr in cexpr.take_list().unwrap() {
            exprs.push(self.analyze(cexpr)?);
        }
        Ok(Expr::Invoke(exprs))
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