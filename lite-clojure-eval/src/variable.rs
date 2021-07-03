use std::sync::Arc;

pub enum Variable {
    Int(i64),
    Float(f64),
    Bool(bool),
    Symbol(Symbol),
    String(Arc<String>),
    Nil,
}

pub struct Symbol {
    is_val:bool,
    var_name:Arc<String>,
    stack_index:usize,
}

impl Symbol {
    pub fn val(name:Arc<String>,index:usize) -> Symbol {
        Symbol {
            is_val: true,
            var_name: name,
            stack_index: index
        }
    }
}