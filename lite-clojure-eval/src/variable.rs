use std::sync::Arc;

#[derive(Debug)]
pub enum Variable {
    Int(i64),
    Float(f64),
    Bool(bool),
    Symbol(Symbol),
    String(Arc<String>),
    Nil,
}

#[derive(Debug)]
pub struct Symbol {
    is_global:bool,
    pub var_name:Arc<String>,
    stack_index:usize,
}

impl Symbol {
    pub fn is_global(&self) -> bool { self.is_global }
    pub fn val(name:Arc<String>,index:usize) -> Symbol {
        Symbol {
            is_global: true,
            var_name: name,
            stack_index: index
        }
    }
}