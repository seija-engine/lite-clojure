use std::path::PathBuf;
use crate::{Variable, exec_context::ExecContext,EvalError, module::EvalModules};


pub struct EvalRT {
   main_ctx:ExecContext,
   modules:EvalModules
}

impl EvalRT {

    pub fn new() -> EvalRT {
        EvalRT { 
            main_ctx:ExecContext::new(),
            modules:EvalModules::default()
        }
    }

    pub fn init(&mut self) {
        self.modules.init();
    }

    pub fn add_search_path<P:Into<PathBuf>>(&mut self,path:P) {
        self.modules.search_path.push(path.into());
    }

    pub fn invoke_func(&mut self,fn_name:&str,args:Vec<Variable>) -> Result<Variable,EvalError> {
       self.main_ctx.invoke_func(fn_name, args, &mut self.modules)
    }

    pub fn invoke_func2(&mut self,fn_var:&Variable,args:Vec<Variable>) -> Result<Variable,EvalError> {
        self.main_ctx.invoke_func2(fn_var, args, &mut self.modules)
    }
    
    pub fn eval_string(&mut self,file_name:String,code_string:&str) -> Option<Variable> {
       self.main_ctx.eval_string(file_name, code_string, &mut self.modules)
    }

    pub fn eval_file(&mut self,path:&str) -> Option<Variable> {
        let code = std::fs::read_to_string(path).unwrap();
        self.eval_string(String::from(path), &code)
    }

    pub fn main_context(&mut self) -> &mut ExecContext {
        &mut self.main_ctx
    }

    pub fn global_context(&mut self) -> &mut ExecContext {
        &mut self.modules.prelude
    }

    pub fn add_module(&mut self,mod_name:&str,code_string:&str) {
        self.modules.require_mod_str(mod_name,code_string)
    }
}