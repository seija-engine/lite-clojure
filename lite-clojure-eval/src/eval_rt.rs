use crate::Variable;
use crate::EvalError;
use crate::buildin_fn;
use crate::sym_scope::SymbolScopes;
use crate::variable::Function;
use crate::variable::Symbol;
use crate::variable::VariableRef;
use std::sync::Arc;
use lite_clojure_parser::expr::Expr;
use lite_clojure_parser::ast::ASTModule;
use lite_clojure_parser::ast::parse_ast;
use lite_clojure_parser::cexpr::Number;
use lite_clojure_parser::value::{Symbol as ASTSymbol};

#[derive(Debug)]
struct Callstack {
    pub(crate) index: usize,
}


pub struct EvalRT {
    pub(crate) stack: Vec<Variable>,
    call_stack:Vec<Callstack>,
    sym_maps:SymbolScopes
}

impl EvalRT {

    pub fn new() -> EvalRT {
        EvalRT { 
            stack:vec![],
            sym_maps:SymbolScopes::new(),
            call_stack:vec![Callstack {index : 0 }]
        }
    }

    pub fn init(&mut self) {
        self.push_native_fn("print", |rt,args| buildin_fn::print(rt,args,false));
        self.push_native_fn("println", |rt,args| buildin_fn::print(rt,args,true));
        self.push_native_fn("+", buildin_fn::num_add);
        self.push_native_fn("-", buildin_fn::num_sub);
        self.push_native_fn("*", buildin_fn::num_mul);
        self.push_native_fn("/", buildin_fn::num_div);
    }

    pub fn push_native_fn(&mut self,name:&str,f:fn(&EvalRT,Vec<VariableRef>) -> Variable ) {
        let f_var = Variable::Function(Function::NativeFn(f));
        self.stack.push(f_var);
        let fn_sym = Symbol::val(Arc::new(name.to_string()), self.stack.len() - 1);
        self.sym_maps.top_scope().push_sym(fn_sym);
    }

    pub fn eval_string(&mut self,file_name:String,code_string:&str) {
        let ast_module = parse_ast(file_name, code_string).unwrap();
        self.eval_ast_module(ast_module);
    }

    pub fn eval_ast_module(&mut self,ast_module:ASTModule) {
        for expr in ast_module.exprs {
            self.eval_expr(&expr,false).unwrap();
        }
    }

    fn eval_expr(&mut self,expr:&Expr,is_push_stack:bool) -> Result<(),EvalError> {
        match expr {
            Expr::Boolean(b) => if is_push_stack {self.stack.push(Variable::Bool(*b))},
            Expr::Nil => if is_push_stack { self.stack.push(Variable::Nil) },
            Expr::Number(Number::Int(inum)) => {
                if is_push_stack { self.stack.push(Variable::Int(*inum)) };
            },
            Expr::Number(Number::Float(fnum)) => {
                if is_push_stack { self.stack.push(Variable::Float(*fnum)) };
            },
            Expr::String(str) => {
                if is_push_stack { self.stack.push(Variable::String(Arc::new(str.to_owned()))) };
            },
            Expr::Def(doc,sym,val) => {
                self.eval_def(sym, val, doc)?;
            },
            Expr::Invoke(lst) => { self.eval_fn(lst,is_push_stack)?;  },
            Expr::Symbol(sym) => { self.relsove_sym(sym)?; },
            
            //Expr::Invoke(lst) => self.eval_fn(lst),
            //Expr::Symbol(sym) => Ok(self.relsove_sym(sym)),
            _ => todo!()
        }
        Ok(())
    }

    fn relsove_sym(&mut self,sym:&ASTSymbol) -> Result<(),EvalError> {
        let last_scope = self.sym_maps.last_scope_ref();
        let n = last_scope.find(&sym.name);
        match n {
            None => Err(EvalError::NotFoundSymbol(sym.name.clone())),
            Some(s) => {
                self.stack.push(Variable::Ref(VariableRef(s.index())));
                Ok(())
            }
        }
    }

    fn eval_def(&mut self,sym:&ASTSymbol,val:&Option<Box<Expr>>,_doc:&Option<String>) -> Result<(),EvalError> {
        match val {
            None =>self.stack.push(Variable::Nil),
            Some(e) => { self.eval_expr(&*e,true)?; },
        };
       
        let idx = self.stack.len() - 1;
        let sym_name = Arc::new(sym.name.to_string());
        let var_sym = Symbol::val(sym_name, idx);
        
        self.sym_maps.last_scope().push_sym(var_sym);
        Ok(())
    }

    fn eval_fn(&mut self,lst:&Vec<Expr>,is_push_stack:bool) -> Result<(),EvalError> {
        if lst.len() == 0 {
            return Err(EvalError::ZeroFnList);
        };
        self.enter_function();
        for e in lst.iter() {
            self.eval_expr(e,true)?;       
        }
        let stack_len = self.stack.len();
        let fn_index = stack_len - lst.len();
       
        let fn_var = self.get_var(&self.stack[fn_index]);
       
        match fn_var {
            Variable::Function(f) => {
                let cur_idx = fn_index + 1;
                let mut args:Vec<VariableRef> = vec![];
                for i in 0..(lst.len() - 1) {
                   
                    args.push(VariableRef(cur_idx + i));
                }
                let ret = f.call(self, args);
                if is_push_stack { self.stack.push(ret) };
            },
            _ => {
                self.exit_function(false);
                return Err(EvalError::ListFirstMustFunction)
            }
        }
        self.exit_function(is_push_stack);
        Ok(())
    }

    fn enter_function(&mut self) {
        let new_callstack = Callstack {index: self.stack.len()};
        self.call_stack.push(new_callstack);
    }

    fn exit_function(&mut self,keep_last:bool) {
        let last_index = self.call_stack.last().unwrap().index;
        let last = self.stack.drain(last_index..).last();
        if keep_last {
            if let Some(v) = last {
                self.stack.push(v);
            }
        }
        self.call_stack.pop();
    }


    pub fn get_var<'a>(&'a self,var:&'a Variable) -> &'a Variable {
        match var {
            Variable::Ref(r) => self.get_var(&self.stack[r.0]),
            v => v 
        }
    }
}



#[test]
fn test_eval() {
    let code = r#"
      (def test-num 114514)
      
      (defn fuck [var]
        (println "fuck" var)
        var
      )
    "#;
    let mut rt = EvalRT::new();
    rt.init();
    rt.eval_string(String::from("test"),code);
    
    
}