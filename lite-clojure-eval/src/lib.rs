mod eval_rt;
mod variable;
mod error;
mod sym_scope;
mod buildin_fn;
mod value;
mod module;
mod exec_context;

pub use variable::{Variable,GcRefCell,ExecScope};
pub use lite_clojure_parser as parser;
pub use eval_rt::EvalRT;
pub use error::EvalError;
use anyhow::{Result};

pub fn run_native_fn(name:&str,scope:&mut ExecScope,args:Vec<Variable>,f:fn(&mut ExecScope,args:Vec<Variable>) -> Result<Variable>) -> Variable {
    match f(scope,args) {
        Ok(var) => var,
        Err(err) => {
            log::error!("native func {} error:{}",name,err);
            Variable::Nil
        }
    }
}