use crate::Variable;
use crate::EvalError;
use crate::variable::Symbol;
use std::sync::Arc;
use lite_clojure_parser::expr::Expr;
use lite_clojure_parser::ast::ASTModule;
use lite_clojure_parser::ast::parse_ast;
use lite_clojure_parser::cexpr::Number;
use lite_clojure_parser::value::{Symbol as ASTSymbol};
struct CallStack {
    fn_name: Arc<String>,
    pub(crate) index: usize,
}

struct EvalRT {
    pub stack: Vec<Variable>,
}

impl EvalRT {

    pub fn new() -> EvalRT {
        EvalRT { 
            stack:vec![] 
        }
    }

    pub fn eval_string(&mut self,file_name:String,code_string:&str) {
        let ast_module = parse_ast(file_name, code_string).unwrap();
        self.eval_ast_module(ast_module);
    }

    pub fn eval_ast_module(&mut self,ast_module:ASTModule) {
        for expr in ast_module.exprs {
            self.eval_expr(&expr);
        }
    }

    fn eval_expr(&mut self,expr:&Expr) -> Result<Variable,EvalError> {
        match expr {
            Expr::Boolean(b) => Ok(Variable::Bool(*b)),
            Expr::Nil => Ok(Variable::Nil),
            Expr::Number(Number::Int(inum)) => Ok(Variable::Int(*inum)),
            Expr::Number(Number::Float(fnum)) => Ok(Variable::Float(*fnum)),
            Expr::String(str) => Ok(Variable::String(Arc::new(str.to_string()))),
            Expr::Invoke(lst) => self.eval_fn(lst),
            Expr::Def(doc,sym,val) => self.eval_def(sym, val,doc),
            _ => todo!()
        }
    }

    fn eval_def(&mut self,sym:&ASTSymbol,val:&Option<Box<Expr>>,doc:&Option<String>) -> Result<Variable,EvalError> {
        let eval_var = match val {
            None => Variable::Nil,
            Some(e) => self.eval_expr(&*e)?
        };
        let idx = self.stack.len();
        let sym_name = Arc::new(sym.name.to_string());
        let var_sym = Symbol::val(sym_name, idx);
        self.stack.push(eval_var);
        Ok(Variable::Nil)
    }

    fn eval_fn(&mut self,lst:&Vec<Expr>) -> Result<Variable,EvalError> {
        //let fn_var = self.eval_expr(lst[0])?;
        todo!()
    }
}


#[test]
fn test_eval() {
    let code = r#"
      (def n1 114514)
      (def n2 999)
      (+ n1 n2)
    "#;
    let mut rt = EvalRT::new();
    rt.eval_string(String::from("test"),code);
}