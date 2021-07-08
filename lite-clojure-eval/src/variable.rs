use std::{cell::RefCell, collections::HashMap, fmt::{Debug, Formatter, Write}, rc::Rc, sync::Arc, usize};
use gc::{Gc,GcCell,Finalize,Trace,GcCellRef,GcCellRefMut };
use lite_clojure_parser::expr::Expr;

use crate::eval_rt::{EvalRT};

#[derive(Debug,Clone,Finalize,Trace)]
pub struct  GcRefCell<T:Trace + Finalize + 'static>(Gc<GcCell<T>>);

impl<T> GcRefCell<T> where T:Trace + Finalize {

    pub fn new(val:T) -> GcRefCell<T> {
        GcRefCell(Gc::new(GcCell::new(val)))
    }

    pub fn borrow(&self) -> GcCellRef<'_,T> {
        let b:&GcCell<T> = &self.0;
        b.borrow()
    }

    pub fn borrow_mut(&self) -> GcCellRefMut<'_,T> {
        let b:&GcCell<T> = &self.0;
        b.borrow_mut()
    }
}

#[derive(Debug,Clone,Finalize,Trace)]
pub enum Variable {
    Int(i64),
    Float(f64),
    Bool(bool),
    Symbol(Symbol),
    String(GcRefCell<String>),
    Function(Gc<Function>),
    //Ref(VariableRef),
    Array(GcRefCell<Vec<Variable>>),
    Var(String),
    Char(char),
    Nil,
}

impl Variable {
    pub fn show_str(&self,rt:&EvalRT) -> String {
        match self {
            Variable::Int(v) => format!("{}",v),
            Variable::Bool(v) => format!("{}",v),
            Variable::Float(v) => format!("{}",v),
            Variable::Symbol(v) => format!("{}",v.var_name),
            Variable::String(v) => format!("{}",v.borrow()),
            Variable::Char(chr) => format!("'{}'",chr),
            Variable::Function(_) => String::from("function"),
            Variable::Nil => "nil".to_string(),
            Variable::Var(s) =>format!("#'{}",s),
            Variable::Array(lst) => {
                let mut lst_string:String = String::default();
                let lst_ref = lst.borrow();
                for idx in 0..lst_ref.len() {
                    let elem = &lst_ref[idx];
                    lst_string.push_str(elem.show_str(rt).as_str());
                    if idx < lst_ref.len() - 1 {
                        lst_string.push(' ');
                    }
                };
                format!("[{}]",lst_string)
            }
        }
    }

    pub fn cast_int(&self,_rt:&EvalRT) -> Option<i64> {
        match self {
            Variable::Int(n) => Some(*n),
            _ => None
        }
    }

    pub fn cast_float(&self,_rt:&EvalRT) -> Option<f64> {
        match self {
            Variable::Float(n) => Some(*n),
            Variable::Int(n) => Some(*n as f64),
            _ => None
        }
    }
    pub fn cast_bool(&self,_rt:&EvalRT) -> Option<bool> {
        match self {
            Variable::Bool(n) => Some(*n),
            _ => None
        }
    }

    pub fn cast_vec(&self,_rt:&EvalRT) -> Option<GcRefCell<Vec<Variable>>> {
        match self {
            Variable::Array(arr) => Some(arr.clone()),
            _ => None
        }
    }

    pub fn cast_var(&self,_rt:&EvalRT) -> Option<String> {
        match self {
            Variable::Var(s) => Some(s.clone()),
            _ => None
        }
    }
}


#[derive(Debug,Clone,Finalize,Trace)]
pub struct Symbol {
    pub var_name:String,
    stack_index:usize,
    pub bind_value:Option<Rc<GcCell<Variable>>>
}

impl Symbol {
    pub fn val(name:String,index:usize) -> Symbol {
        Symbol {
            var_name: name,
            stack_index: index,
            bind_value:None
        }
    }

    pub fn index(&self) -> usize {
        self.stack_index
    }

    pub fn set_index(&mut self,idx:usize) {
        self.stack_index = idx;
    }
}



#[derive(Finalize,Trace)]
pub  enum Function {
    NativeFn(#[unsafe_ignore_trace] fn(&mut EvalRT,args:Vec<Variable>) -> Variable),
    ClosureFn(ClosureData)
}

#[derive(Debug,Finalize,Trace)]
pub struct ClosureData {
    pub args:Vec<Symbol>,
    #[unsafe_ignore_trace]
    pub body:Vec<Expr>,
    pub cap_vars:Option<HashMap<String,Symbol>>
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Function")
    }
}