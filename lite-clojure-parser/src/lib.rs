pub mod cexpr;
pub mod expr;
pub mod cst;
pub mod ast;
mod lex_string;
pub mod errors;
pub mod utils;
pub mod value;
pub mod env;
pub mod meta;

use std::{sync::Mutex};

use cexpr::CExpr;
use lazy_static::lazy_static;

type MacroFunc = fn(&mut CExpr);

lazy_static! {
    static ref GLOBAL_MACRO_HOOKS:Mutex<Vec<MacroFunc>> = Mutex::new(Vec::new());
}

pub fn add_macro_func(func:MacroFunc) {
    GLOBAL_MACRO_HOOKS.lock().unwrap().push(func);
}