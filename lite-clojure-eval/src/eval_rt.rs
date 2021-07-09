use crate::Variable;
use crate::EvalError;
use crate::buildin_fn;
use crate::sym_scope::SymbolScope;
use crate::sym_scope::SymbolScopes;
use crate::variable::ClosureData;
use crate::variable::Function;
use crate::variable::GcRefCell;
use crate::variable::Symbol;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::usize;
use gc::Gc;
use gc::GcCell;
use lite_clojure_parser::expr::Expr;
use lite_clojure_parser::ast::ASTModule;
use lite_clojure_parser::ast::parse_ast;
use lite_clojure_parser::cexpr::Number;
use lite_clojure_parser::value::{Symbol as ASTSymbol};

#[derive(Debug,Clone)]
struct Callstack {
    pub need_loop:bool,
    is_recur:bool,
    is_let:bool,
    pub(crate) index: usize,
}


pub struct EvalRT {
    pub(crate) stack: Vec<Variable>,
    call_stack:Vec<Callstack>,
    pub sym_maps:SymbolScopes
}

impl EvalRT {

    pub fn new() -> EvalRT {
        EvalRT { 
            stack:vec![],
            sym_maps:SymbolScopes::new(),
            call_stack:vec![Callstack {index : 0 ,need_loop:false,is_recur:false,is_let:false}]
        }
    }

    pub fn init(&mut self) {
        self.push_native_fn("var-set", buildin_fn::var_set);
        self.push_native_fn("print", |rt,args| buildin_fn::print(rt,args,false));
        self.push_native_fn("println", |rt,args| buildin_fn::print(rt,args,true));
        self.push_native_fn("+", buildin_fn::num_add);
        self.push_native_fn("-", buildin_fn::num_sub);
        self.push_native_fn("*", buildin_fn::num_mul);
        self.push_native_fn("/", buildin_fn::num_div);

        self.push_native_fn("<", buildin_fn::num_lt);
        self.push_native_fn("<=", buildin_fn::num_le);
        self.push_native_fn(">", buildin_fn::num_gt);
        self.push_native_fn(">=", buildin_fn::num_ge);

        self.push_native_fn("nth", buildin_fn::nth);
        let core_code = include_str!("core.clj");
        self.eval_string(String::from("core.clj"), core_code);
    }

    pub fn push_native_fn(&mut self,name:&str,f:fn(&mut EvalRT,Vec<Variable>) -> Variable ) {
        let f_var = Variable::Function(Gc::new(Function::NativeFn(f)));
        self.stack.push(f_var);
        let fn_sym = Symbol::val(name.to_string(), self.stack.len() - 1);
        self.sym_maps.top_scope().push_sym(fn_sym);
    }

    pub fn eval_string(&mut self,file_name:String,code_string:&str) {
        let ast_module = parse_ast(file_name, code_string).unwrap();
        self.eval_ast_module(ast_module);
    }

    pub fn eval_file(&mut self,path:&str) {
        let code = std::fs::read_to_string(path).unwrap();
        self.eval_string(String::from(path), &code);
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
                if is_push_stack { self.stack.push(Variable::String(GcRefCell::new(str.to_owned()))) };
            },
            Expr::Def(doc,sym,val) => {
                self.eval_def(sym, val, doc)?;
            },
            Expr::Invoke(lst) => { self.eval_invoke(lst,is_push_stack)?;  },
            Expr::Symbol(sym) => { self.relsove_sym(sym)?; },
            Expr::Fn(syms,form) => {self.eval_fn(syms, form)?; },
            Expr::Let(binds,body,is_loop) => { self.eval_let(binds,body,*is_loop,is_push_stack)?; }
            Expr::Body(lst) => {self.eval_body(lst)?; },
            Expr::If(cond,expr_true,expr_false) => {self.eval_if(cond,expr_true,expr_false,is_push_stack)?; },
            Expr::Vector(lst) => {self.eval_vector(lst, is_push_stack)?; },
            Expr::QuoteVar(s) => if is_push_stack { 
                let str_name = s.name.to_owned();
                let var =  Variable::Var(str_name );
                if is_push_stack {self.stack.push(var); }
            },
            Expr::Recur(args) => { self.eval_recur(args)?; },
            //Expr::Invoke(lst) => self.eval_fn(lst),
            //Expr::Symbol(sym) => Ok(self.relsove_sym(sym)),
            _ => todo!()
        }
        Ok(())
    }

    fn eval_recur(&mut self,args:&Vec<Expr>)  -> Result<(),EvalError> {
        let call_index = {
            let callstack = self.find_last_recur_stack().unwrap();
            callstack.need_loop = true;
            if callstack.is_let {callstack.index } else {callstack.index + 1 }
           
        };
       
        for arg in args {
            self.eval_expr(arg, true)?;
        }
        let drain_index = self.stack.len() - args.len();
        let eval_args:Vec<Variable> = self.stack.drain(drain_index..).collect();
        let mut idx = 0;

        for arg in eval_args {
            self.stack[call_index + idx] = arg;
            idx += 1;
        }
        Ok(())
    }

    fn eval_body(&mut self,lst:&Vec<Expr>) -> Result<(),EvalError> {
        for idx in 0..lst.len() {
            self.eval_expr(&lst[idx], idx == lst.len() - 1)?;
        }
        Ok(())
    }

    fn eval_vector(&mut self,lst:&Vec<Expr>,is_push_stack:bool) -> Result<(),EvalError> {
        let idx = self.stack.len();
        for idx in 0..lst.len() {
            self.eval_expr(&lst[idx], true)?;
        }

        let var_lst:Vec<Variable> = self.stack.drain(idx..).collect();
        if is_push_stack { self.stack.push(Variable::Array(GcRefCell::new(var_lst))) };
        Ok(())
    }

    fn enter_let(&mut self,need_loop:bool) {
        self.sym_maps.last_scope().push_let();
        let new_callstack = Callstack {index: self.stack.len(),need_loop : need_loop,is_recur:need_loop,is_let:true};
        self.call_stack.push(new_callstack);
    }

    fn exit_let(&mut self,keep_last:bool) {
        self.exit_callstack(keep_last);
        self.sym_maps.last_scope().pop_let();
    }

    fn eval_if(&mut self,cond:&Expr,expr_true:&Expr,expr_false:&Expr,is_push_stack:bool) -> Result<(),EvalError> {
        self.eval_expr(cond, true)?;
        
        let last_var = self.stack.pop().unwrap();
        let is_true = last_var.cast_bool(self).unwrap();  
        if is_true {
            self.eval_expr(expr_true, is_push_stack)?;
        } else {
            self.eval_expr(expr_false, is_push_stack)?;
        }
        Ok(())
    }

    fn eval_let(&mut self,binds:&Vec<Expr>,body:&Box<Expr>,is_loop:bool,is_push_stack:bool) -> Result<(),EvalError> {
        self.enter_let(is_loop);
        //let 放入let变量
        for idx in 0..binds.len() / 2 {
            let index = idx * 2;
            let s = &binds[index];
            self.eval_expr(&binds[index + 1], true)?;
            match s {
                Expr::Symbol(s) => {
                   let new_sym = Symbol::val(s.name.clone(), self.stack.len() - 1);
                   self.sym_maps.last_scope().push_sym(new_sym);
                }
                _ => {}
            }
        }
        
        if is_loop {
           let body_index = self.stack.len();
           self.eval_expr(body, true)?;
           let mut need_loop = self.call_stack.last().unwrap().need_loop;
           while need_loop {
              self.call_stack.last_mut().unwrap().need_loop = false;
              self.eval_expr(body, true)?;
              need_loop = self.call_stack.last().unwrap().need_loop;
              if need_loop {
                  self.stack.drain(body_index..);
              } else {
                 let last = self.stack.drain(body_index..).last().unwrap();
                 self.stack.push(last);
              }
           }
        } else {
            self.eval_expr(body, is_push_stack)?;
        }
       
        self.exit_let(true);
        Ok(())
    }

    fn eval_fn(&mut self,ast_syms:&Vec<ASTSymbol>,form:&Vec<Expr>) -> Result<(),EvalError> {
        let mut syms:Vec<Symbol> = vec![];
        for ast_sym in ast_syms {
            let  sym = Symbol::val(ast_sym.name.clone(), 0);
            syms.push(sym);
        }
        let mut closure_data = ClosureData {args:syms,body:form.clone(),cap_vars: None};
        if let Some(cap_map) = self.az_closure_syms(ast_syms,form)? {
            closure_data.cap_vars = Some(cap_map);
            
        }
        let closure = Gc::new(Function::ClosureFn(closure_data));
        self.stack.push(Variable::Function(closure));
        Ok(())
    }
    /*
      (defn gen-closure [a b]
         (let [var 123]
           (fn [c]
              (let [d 123]
                 (+ a b c var d)
              )
           )
         )
      )
    */
    fn az_closure_syms(&mut self,ast_syms:&Vec<ASTSymbol>,forms:&Vec<Expr>) -> Result<Option<HashMap<String,Symbol>>,EvalError>  {
        let mut fn_scope = SymbolScope::default();
        let mut not_found_syms:HashSet<ASTSymbol> = HashSet::new();
        for sym in ast_syms {
            let sym = Symbol::val(String::from(sym.sym_name()), 0);
            fn_scope.push_sym(sym);
        }
        for expr in forms {
            self.az_expr(&mut fn_scope, expr, &mut not_found_syms)
        }
        
        if not_found_syms.is_empty() {
            return Ok(None);
        }
       
        let mut hash_map:HashMap<String,Symbol> = HashMap::new();
        for ast_sym in not_found_syms {
           if let Some(sym) = self.sym_maps.deep_find(&ast_sym.name) {
               let var_ref = self.stack[sym.index()].clone();
               let clone_var = var_ref;
               let mut new_sym = Symbol::val(ast_sym.name.clone(), 0);
               new_sym.bind_value = Some(Rc::new(GcCell::new(clone_var)));
               hash_map.insert(ast_sym.name.clone(), new_sym);
           }
        }
        Ok(Some(hash_map))
    }

    fn az_expr(&mut self,scope:&mut SymbolScope,expr:&Expr,not_found_syms:&mut HashSet<ASTSymbol>) {
       
        match expr {
            Expr::Body(lst) => {
                lst.iter().for_each(|e|  self.az_expr(scope, e, not_found_syms))
            },
            Expr::If(cond,e_true,e_false) => {
                self.az_expr(scope, cond, not_found_syms);
                self.az_expr(scope, e_true, not_found_syms);
                self.az_expr(scope, e_false, not_found_syms);
            },
            Expr::Invoke(froms) => {
                froms.iter().for_each(|e|  self.az_expr(scope, e, not_found_syms))
            },
            Expr::Vector(lst) => {
                lst.iter().for_each(|e|  self.az_expr(scope, e, not_found_syms))
            },
            Expr::Let(binds,body,_) => {
                scope.push_let();
                for idx in 0..binds.len() / 2 {
                    let cur_expr = binds[idx * 2].clone();
                    let ast_sym = cur_expr.case_sym().unwrap();
                    let sym = Symbol::val(String::from(ast_sym.sym_name()), 0);
                    scope.push_sym(sym);
                }
                self.az_expr(scope, body, not_found_syms);
                scope.pop_let();
            },
            Expr::Symbol(sym) => {
                let top_scope = self.sym_maps.top_scope_ref();
                if scope.find(&sym.name).is_none() && top_scope.find(&sym.name).is_none() {
                    if !not_found_syms.contains(&sym) {not_found_syms.insert(sym.clone()); };
                }
            },
            _ => ()
        }
    }

    fn relsove_sym(&mut self,sym:&ASTSymbol) -> Result<(),EvalError> {
        let  n = self.sym_maps.find_local_or_top(&sym.name);
       
        match n {
            None => Err(EvalError::NotFoundSymbol(sym.name.clone())),
            Some(s) => {
                if let Some(inner) = &s.bind_value {
                    let cell:&GcCell<Variable> = inner;
                    let var = cell.borrow().clone();
                    self.stack.push(var);
                    return Ok(());
                }
                let clone_var = self.stack[s.index()].clone();
                self.stack.push(clone_var);
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
        let var_sym = Symbol::val(sym.name.to_string(), idx);
        
        self.sym_maps.last_scope().push_sym(var_sym);
        Ok(())
    }


    fn eval_invoke(&mut self,lst:&Vec<Expr>,is_push_stack:bool) -> Result<(),EvalError> {
        if lst.len() == 0 {
            return Err(EvalError::ZeroFnList);
        };
        let start_index = self.stack.len();
        for e in lst.iter() {
            self.eval_expr(e,true)?;       
        }
        
       
        let stack_len = self.stack.len();
        let fn_index = stack_len - lst.len();
        let func = {
            let fn_var = &self.stack[fn_index];
            match fn_var {
                Variable::Function(f) => f.clone(),
                _ => {
                    return Err(EvalError::ListFirstMustFunction)
                }
            }
        };
        

        let cur_idx = fn_index + 1;
        let mut args:Vec<Variable> = vec![];
        for i in 0..(lst.len() - 1) {
            let var = self.stack[cur_idx + i].clone();
            args.push(var);
        }

        

        let func_ref:&Function = &func;
        match func_ref {
            Function::NativeFn(nf) => {
                self.enter_function(start_index);
                let ret = nf(self,args);
                if is_push_stack { self.stack.push(ret) };
            },
            Function::ClosureFn(closure_data) => {
                if args.len() != closure_data.args.len() {
                    return Err(EvalError::FunctionArgCountError);
                }
                self.enter_function(start_index);
                let mut  var_idx = 1;
                //把函数参数push入栈
                for sym in &closure_data.args {
                    let new_sym = Symbol::val(sym.var_name.to_string(), fn_index + var_idx); 
                    self.sym_maps.last_scope().push_sym(new_sym);
                    var_idx += 1;
                }

                //把闭包捕获的变量入栈
                if let Some(cap_vars) = &closure_data.cap_vars {
                    let vars = cap_vars;
                    for (_,v) in vars.iter() {
                        self.sym_maps.last_scope().push_sym(v.clone());
                    }
                }
                let body_index = self.stack.len();
                self.eval_closure(closure_data)?;
                let mut need_loop = self.call_stack.last().unwrap().need_loop;
                while need_loop {
                    self.call_stack.last_mut().unwrap().need_loop = false;
                    self.eval_closure(closure_data)?;
                    need_loop = self.call_stack.last().unwrap().need_loop;
                    let last = self.stack.drain(body_index..).last();
                    if need_loop == false {
                        
                        self.stack.push(last.unwrap());
                    }
                }
            }
        };
      
        self.exit_function(is_push_stack);
        Ok(())
    }

    fn eval_closure(&mut self,closure_data:&ClosureData) -> Result<(),EvalError> {
        let mut idx = 0;
        let form_len = closure_data.body.len() - 1;
        for form_expr in &closure_data.body {
           self.eval_expr(&form_expr,form_len == idx )?;
           idx += 1;
        }
        Ok(())
    }

    

    fn enter_function(&mut self,start_index:usize) {
        let new_callstack = Callstack {index: start_index,need_loop:false,is_recur:true,is_let:false};
        self.call_stack.push(new_callstack);
        self.sym_maps.push_scope();
    }

    fn exit_callstack(&mut self,keep_last:bool) {
        let last_push = if keep_last {
            Some(self.stack.last().unwrap().clone())
        } else { None };

        let last_index = self.call_stack.last().unwrap().index;
        self.stack.drain(last_index..);
        if let Some(last_val) = last_push {
            self.stack.push(last_val);
        }
        self.call_stack.pop();
    }

    fn exit_function(&mut self,keep_last:bool) {
        self.exit_callstack(keep_last);   
        self.sym_maps.pop_scope();
       
    }
    
    fn find_last_recur_stack(&mut self) -> Option<&mut Callstack> {
        for cs in self.call_stack.iter_mut().rev() {
            if cs.is_recur {
                return  Some(cs);
            }
        }
        None
    }
}