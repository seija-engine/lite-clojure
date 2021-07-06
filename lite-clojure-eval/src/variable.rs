use std::{fmt::{Debug, Formatter, Write}, sync::Arc, usize};
use lite_clojure_parser::expr::Expr;

use crate::eval_rt::{EvalRT};

#[derive(Debug)]
pub enum Variable {
    Int(i64),
    Float(f64),
    Bool(bool),
    Symbol(Symbol),
    String(Arc<String>),
    Function(Arc<Function>),
    Ref(VariableRef),
    Nil,
}

impl Variable {
    pub fn show_str(&self,rt:&EvalRT) -> String {
        match self {
            Variable::Int(v) => format!("{}",v),
            Variable::Bool(v) => format!("{}",v),
            Variable::Float(v) => format!("{}",v),
            Variable::Symbol(v) => format!("{}",v.var_name),
            Variable::String(v) => format!("{}",v),
            Variable::Function(_) => String::from("function"),
            Variable::Nil => "nil".to_string(),
            Variable::Ref(r) => r.get_ref(rt).show_str(rt)
        }
    }

    pub fn cast_int(&self,rt:&EvalRT) -> Option<i64> {
        match rt.get_var(self) {
            Variable::Int(n) => Some(*n),
            _ => None
        }
    }

    pub fn cast_float(&self,rt:&EvalRT) -> Option<f64> {
        match rt.get_var(self) {
            Variable::Float(n) => Some(*n),
            Variable::Int(n) => Some(*n as f64),
            _ => None
        }
    }
}


#[derive(Debug,Clone)]
pub struct Symbol {
    pub is_global:bool,
    pub var_name:Arc<String>,
    stack_index:usize,
}

impl Symbol {
    pub fn is_global(&self) -> bool { self.is_global }
    pub fn val(name:Arc<String>,index:usize,is_global:bool) -> Symbol {
        Symbol {
            is_global: is_global,
            var_name: name,
            stack_index: index
        }
    }

    pub fn index(&self) -> usize {
        self.stack_index
    }

    pub fn set_index(&mut self,idx:usize) {
        self.stack_index = idx;
    }
}

#[derive(Debug)]
pub struct VariableRef(pub usize);

impl VariableRef {
    pub fn get_ref<'a>(&self,rt:&'a EvalRT) -> &'a Variable {
        rt.get_var(&rt.stack[self.0])
    }
   
}

pub  enum Function {
    NativeFn(fn(&EvalRT,args:Vec<VariableRef>) -> Variable),
    ClosureFn(Vec<Symbol>,Vec<Expr>)
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Function")
    }
}