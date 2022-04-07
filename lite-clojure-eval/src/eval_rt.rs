use std::path::PathBuf;

use lite_clojure_parser::ast::parse_ast;

use crate::{Variable, exec_context::ExecContext, buildin_fn, EvalError, module::EvalModules};

#[derive(Debug,Clone)]
struct Callstack {
    pub need_loop:bool,
    is_recur:bool,
    is_let:bool,
    pub(crate) index: usize,
}


pub struct EvalRT {
   global_ctx:ExecContext,
   modules:EvalModules
}

impl EvalRT {

    pub fn new() -> EvalRT {
        EvalRT { 
            global_ctx:ExecContext::new(),
            modules:EvalModules::default()
        }
    }

    pub fn init(&mut self) {
        self.modules.init();
    }

    pub fn set_search_path<P:Into<PathBuf>>(&mut self,path:P) {
        self.modules.search_path = path.into();
    }

    pub fn invoke_func(&mut self,fn_name:&str,args:Vec<Variable>) -> Result<Variable,EvalError> {
       self.global_ctx.invoke_func(fn_name, args, &mut self.modules)
    }

 
    pub fn eval_string(&mut self,file_name:String,code_string:&str) -> Option<Variable> {
       self.global_ctx.eval_string(file_name, code_string, &mut self.modules)
    }

    pub fn eval_file(&mut self,path:&str) -> Option<Variable> {
        let code = std::fs::read_to_string(path).unwrap();
        self.eval_string(String::from(path), &code)
    }

    pub fn global(&mut self) -> &mut ExecContext {
        &mut self.global_ctx
    }
}