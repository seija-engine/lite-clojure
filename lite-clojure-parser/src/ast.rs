use crate::errors::CSTError;

use super::{cexpr::{CExpr}, errors::{ASTError}, expr::Expr, meta::MetaTable, value::Symbol};
use super::cst::ParseCST;
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

    fn translate_cexpr(&mut self,mut cexpr:CExpr) {
        //TODO 宏展开
        self.hand_macro_expr(&mut cexpr);

        if let Some(az) = self.analyze(cexpr) {
            match az {
               Ok(v) => {
                   self.exprs.push(v)
               },
               Err(err) => { dbg!(err); }
            }
        }
    }

    fn hand_macro_list(&mut self,lst:&mut Vec<CExpr>) {
        for e in lst {
            self.hand_macro_expr(e);
        }
    }
    fn hand_macro_expr(&mut self,expr:&mut CExpr) {
        match expr {
            CExpr::List(lst) => {
                match lst.first() {
                    Some(CExpr::Symbol(sym)) => {
                        match sym.name.as_str() {
                            "defn" => self.ex_defn(lst),   
                            _=> { self.hand_macro_list(lst);  }
                        }
                    }
                    _=> { self.hand_macro_list(lst);  }
                }
            }
            _ => ()
        }
        
    }


    fn ex_defn(&mut self,lst:&mut Vec<CExpr>)  {
        //(defn fn_name [args] (seq1 ) (seq 2)) -> (def fn_name (fn [args] (seq1) (seq 2)))
        lst.remove(0); //defn
        let name_expr = lst.remove(0);
        let args_expr = lst.remove(0); //[args]
        let mut new_lst:Vec<CExpr> = vec![];
        let def_sym = Symbol::intern(None,String::from("def"));
        new_lst.push(CExpr::Symbol(def_sym));
        new_lst.push(name_expr);

        let fn_sym = Symbol::intern(None,String::from("fn"));
        lst.insert(0, args_expr);
        lst.insert(0, CExpr::Symbol(fn_sym));
       
        new_lst.push(CExpr::List(lst.clone()));
       
      
        *lst = new_lst  
    }



    
    fn analyze(&mut self,cexpr:CExpr) -> Option<Result<Expr,ASTError>> {
       match cexpr {
           CExpr::Nil => Some(Ok(Expr::Nil)),
           CExpr::Boolean(b) => Some(Ok(Expr::Boolean(b))),
           CExpr::Symbol(sym) => Some(self.analyze_sym(sym)),
           CExpr::Number(_raw,num)  => Some(Ok(Expr::Number(num))),
           CExpr::Keyword(key) => Some(Ok(Expr::Keyword(key))),
           CExpr::String(str) => Some(Ok(Expr::String(str))),
           CExpr::Map(lst) => Some(self.analyze_map(lst)),
           CExpr::Vector(lst) => Some(self.analyze_vector(lst)),
           CExpr::QuoteVar(s) => Some(Ok(Expr::QuoteVar(s))),
           CExpr::Comment(_s) => None,
           _ => {
            if cexpr.is_iseq() {
                Some(self.analyze_seq(cexpr))
            } else {
                dbg!(cexpr);
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
                    "fn" => return self.parse_fn_expr(cexpr),
                    "def" =>  return self.parse_def_expr(cexpr),
                    "loop" => return self.parse_let_expr(cexpr,true),
                    "let" =>  return self.parse_let_expr(cexpr, false),
                    "if" => return self.parse_if_expr(cexpr),
                    "case*" => return self.parse_case_expr(),
                    "recur" => return self.parse_recur_expr(cexpr),
                    "do" => {
                        let mut lst = cexpr.take_list().unwrap();
                        lst.remove(0);
                        return self.parse_do_expr_(lst)
                    },
                    _ => return self.parse_invoke(cexpr)
                }
            }
        }
        self.parse_invoke(cexpr)
    }

    fn parse_recur_expr(&mut self,cexpr:CExpr)  -> Result<Expr,ASTError> {
        let mut lst = cexpr.take_list().unwrap();
        lst.remove(0);
        let mut arg_list :Vec<Expr> = vec![];
        for arg in lst {
           if let Some(v) = self.analyze(arg) {
               arg_list.push(v?);
           }
        }
        Ok(Expr::Recur(arg_list))
    }

    fn analyze_map(&mut self,lst:Vec<CExpr>) -> Result<Expr,ASTError> {
        let mut lst_expr:Vec<Expr> = vec![];
        for cexpr in lst {
          if let Some(v) = self.analyze(cexpr) {
              let a_expr = v?;
              lst_expr.push(a_expr);
          }
        }
        Ok(Expr::Map(lst_expr))
    }

    fn analyze_vector(&mut self,lst:Vec<CExpr>) -> Result<Expr,ASTError> {
        let mut lst_expr:Vec<Expr> = vec![];
        for cexpr in lst {
          if let Some(v) = self.analyze(cexpr) {
              let a_expr = v?;
              lst_expr.push(a_expr);
          }
        }
        Ok(Expr::Vector(lst_expr))
    }

    fn parse_case_expr(&mut self) -> Result<Expr,ASTError> {
        todo!()
    }

    fn parse_fn_expr(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
        //(fn [a b c] a) or (fn ([a] a)  ([a b] b))
        let mut lst = cexpr.take_list().unwrap();
        lst.remove(0); //rm fn
        let is_first_vec = lst.first().unwrap().is_vec();
        if is_first_vec {
             let head = self.analyze(lst.remove(0)).unwrap()?;
             let mut flst = head.case_vector().unwrap();
             let sym_lst:Vec<Symbol> = flst.drain(..).map(|f| f.case_sym().unwrap()).collect();
             let mut form_lst:Vec<Expr> = vec![];
             for item in lst {
                 form_lst.push(self.analyze(item).unwrap()?);
             }
             Ok(Expr::Fn(sym_lst,form_lst))
        } else {
            todo!()
        }
    }

    fn parse_if_expr(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
        // (if test then) or (if test then else)
        let mut lst = cexpr.take_list().unwrap();
        if lst.len() > 4 || lst.len() < 3 {
            return Err(ASTError::ErrIf);
        }
        lst.remove(0);
        let test_expr = self.analyze(lst.remove(0)).unwrap().unwrap();
        let then_expr = self.analyze(lst.remove(0)).unwrap().unwrap();
        let else_expr = if lst.len() > 0 {
            self.analyze(lst.remove(0)).unwrap().unwrap()
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
           init_expr = Some(Box::new(self.analyze(lst.remove(0)).unwrap()?));
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
            let sym_expr = self.analyze(cexpr).unwrap()?;
            let val_cexpr = bindings.remove(0);
            let val_expr = self.analyze(val_cexpr).unwrap()?;
            bind_vecs.push(sym_expr);
            bind_vecs.push(val_expr);
        }
        let body_expr = self.parse_do_expr_(lst)?;
        Ok(Expr::Let(bind_vecs,Box::new(body_expr),is_loop))
    }

    fn parse_do_expr_(&mut self,cexprs:Vec<CExpr>) -> Result<Expr,ASTError> {
        let mut exprs:Vec<Expr> = vec![];
        for cexpr in cexprs {
            if let Some(e) = self.analyze(cexpr) {
                exprs.push(e?);
            }
           
        }
        Ok(Expr::Body(exprs))
    }

    fn parse_invoke(&mut self,cexpr:CExpr) -> Result<Expr,ASTError> {
        let mut exprs:Vec<Expr> = vec![];
        for cexpr in cexpr.take_list().unwrap() {
            if let Some(e) = self.analyze(cexpr) {
                exprs.push(e?);
            }
            
        }
        Ok(Expr::Invoke(exprs))
    }
}
 
pub fn parse_ast(file_name:String,code_string:&str) -> Result<ASTModule,ASTError> {
    let mut parser_cst = ParseCST::new(&code_string);
    let cexprs = parser_cst.parse_exprs();
    match cexprs  {
        Ok(cexprs) => {
            let meta_table = parser_cst.take();
            let trans = TranslateToAST::new(file_name.to_string(), cexprs, meta_table);
            let ast_mod = trans.translate();
            Ok(ast_mod)
        }
        Err(err) => {return Err(ASTError::CSTError(err));}
    }
}

#[test]
fn test_trans() {
   use super::cst::ParseCST;
   let file_name = "tests/test.clj";
   let code_string = std::fs::read_to_string(file_name).unwrap();
   let mut parser = ParseCST::new(&code_string);
   let cexprs = parser.parse_exprs().unwrap();
   let meta_table = parser.take();   

   let  trans = TranslateToAST::new(file_name.to_string(), cexprs, meta_table);
   let ast_mod = trans.translate();
   dbg!(ast_mod.exprs);
}