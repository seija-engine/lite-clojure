mod eval_rt;
mod variable;
mod error;
mod sym_scope;
mod buildin_fn;
use std::{cell::RefCell, rc::Rc};

pub use variable::Variable;
pub use eval_rt::EvalRT;
pub use error::EvalError;

#[test]
fn test_gc() {
    
}