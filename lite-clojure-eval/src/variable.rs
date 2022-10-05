use std::{borrow::Borrow, collections::{HashMap}, fmt::{Debug, Formatter}, rc::Rc, usize};
use gc::{Gc,GcCell,Finalize,Trace,GcCellRef,GcCellRefMut };
use lite_clojure_parser::expr::Expr;

use crate::{exec_context::ExecContext, module::EvalModules};

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
    Keyword(GcRefCell<String>),
    Function(Gc<Function>),
    Array(GcRefCell<Vec<Variable>>),
    Map(GcRefCell<HashMap<Variable,Variable>>),
    Var(String),
    Char(char),
    
    UserData(#[unsafe_ignore_trace] *mut u8),
    Nil,
}

impl From<i64> for Variable {
    fn from(src: i64) -> Variable {
        Variable::Int(src)
    }
}

impl From<f64> for Variable {
    fn from(src: f64) -> Variable {
        Variable::Float(src)
    }
}

impl From<bool> for Variable {
    fn from(src: bool) -> Variable {
        Variable::Bool(src)
    }
}

impl From<String> for Variable {
    fn from(src: String) -> Variable {
        Variable::String(GcRefCell::new(src))
    }
}

impl From<&str> for Variable {
    fn from(src: &str) -> Variable {
        Variable::String(GcRefCell::new(src.to_string()))
    }
}

impl From<&[Variable]> for Variable {
    fn from(src: &[Variable]) -> Variable {
        Variable::Array(GcRefCell::new(src.iter().map(|x: &Variable| x.clone()).collect()))
    }
}

impl From<Vec<Variable>> for Variable {
    fn from(src: Vec<Variable>) -> Variable {
        Variable::Array(GcRefCell::new(src))
    }
}

impl From<&[(Variable, Variable)]> for Variable {
    fn from(src: &[(Variable, Variable)]) -> Variable {
        Variable::Map(
            GcRefCell::new(
                src.iter().map(|(k, v): &(Variable, Variable)| (k.clone(), v.clone())).collect()
            )
        )
    }
}

impl From<HashMap<Variable, Variable>> for Variable {
    fn from(src: HashMap<Variable, Variable>) -> Variable {
        Variable::Map(GcRefCell::new(src))
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Variable::Int(v0),Variable::Int(v1)) => v0 == v1,
            (Variable::Float(v0),Variable::Float(v1)) => v0 == v1,
            (Variable::Bool(v0),Variable::Bool(v1)) => v0 == v1,
            (Variable::Char(v0),Variable::Char(v1)) => v0 == v1,
            (Variable::Keyword(v0),Variable::Keyword(v1)) => {
                let str1:&String = &v0.borrow();
                let str2:&String = &v1.borrow();
                str1 == str2
            },
            (Variable::String(v0),Variable::String(v1)) => {
                let str1:&String = &v0.borrow();
                let str2:&String = &v1.borrow();
                str1 == str2
            },
            (Variable::Array(arr),Variable::Array(other_arr)) => {
                let arr_ref:&Vec<Variable> = &arr.borrow();
                let other_ref:&Vec<Variable> = &other_arr.borrow();
                if arr_ref.len() != other_ref.len() {return  false;  }
                for idx in 0..arr_ref.len() {
                    if arr_ref[idx] != arr_ref[idx] {
                        return false;
                    }
                }
                true
            },

            _ => false
        }
    }
}

impl Eq for Variable {}

impl std::hash::Hash for Variable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Variable::Int(v) => v.hash(state),
            Variable::Bool(v) => v.hash(state),
            Variable::Char(v) => v.hash(state),
            Variable::Nil => Variable::Nil.hash(state),
            Variable::String(s) => s.borrow().hash(state),
            Variable::Keyword(s) => s.borrow().hash(state),
            Variable::Symbol(s) => s.borrow().var_name.hash(state),
            _ => 0.hash(state)
        }
    }
}

impl Variable {
    pub fn show_str(&self) -> String {
        match self {
            Variable::Int(v) => format!("{}",v),
            Variable::Bool(v) => format!("{}",v),
            Variable::Float(v) => format!("{}",v),
            Variable::Symbol(v) => format!("{}",v.var_name),
            Variable::String(v) => format!("\"{}\"",v.borrow()),
            Variable::Char(chr) => format!("'{}'",chr),
            Variable::Function(_) => String::from("function"),
            Variable::Nil => "nil".to_string(),
            Variable::Keyword(k) => format!("{}",k.borrow()),
            Variable::Var(s) =>format!("#'{}",s),
            Variable::Map(maps) => {
                let mut kv_string = String::from("{");
                let map:&HashMap<Variable,Variable> =  &maps.borrow();
                for (k,v) in map {
                    kv_string.push_str(k.show_str().as_str());
                    kv_string.push(' ');
                    kv_string.push_str(v.show_str().as_str());
                    kv_string.push(' ');
                }
                kv_string.push('}');
                kv_string
            },
            Variable::Array(lst) => {
                let mut lst_string:String = String::default();
                let lst_ref = lst.borrow();
                for idx in 0..lst_ref.len() {
                    let elem = &lst_ref[idx];
                    lst_string.push_str(elem.show_str().as_str());
                    if idx < lst_ref.len() - 1 {
                        lst_string.push(' ');
                    }
                };
                format!("[{}]",lst_string)
            },
            Variable::UserData(_) =>  String::from("userdata"),
        }
    }

    pub fn cast_int(&self) -> Option<i64> {
        match self {
            Variable::Int(n) => Some(*n),
            _ => None
        }
    }

    pub fn cast_float(&self) -> Option<f64> {
        match self {
            Variable::Float(n) => Some(*n),
            Variable::Int(n) => Some(*n as f64),
            _ => None
        }
    }

    pub fn cast_string(&self) -> Option<GcRefCell<String>> {
        match self {
            Variable::String(n) => Some(n.clone()),
            _ => None
        }
    }

    pub fn cast_keyword(&self) -> Option<GcRefCell<String>> {
        match self {
            Variable::String(n) => Some(n.clone()),
            _ => None
        }
    }

    pub fn cast_bool(&self) -> Option<bool> {
        match self {
            Variable::Bool(n) => Some(*n),
            _ => None
        }
    }

    pub fn cast_vec(&self) -> Option<GcRefCell<Vec<Variable>>> {
        match self {
            Variable::Array(arr) => Some(arr.clone()),
            _ => None
        }
    }

    pub fn cast_map(&self) -> Option<GcRefCell<HashMap<Variable,Variable>>> {
        match self {
            Variable::Map(m) => Some(m.clone()),
            _ => None
        }
    }

    pub fn cast_var(&self) -> Option<String> {
        match self {
            Variable::Var(s) => Some(s.clone()),
            _ => None
        }
    }

    pub fn cast_userdata(&self) -> Option<*mut u8> {
        match self {
            Variable::UserData(s) => Some(*s),
            _ => None
        }
    }

    pub fn cast_function(&self) -> Option<Gc<Function>> {
        match self {
            Variable::Function(f) => Some(f.clone()),
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


pub struct ExecScope<'a> {
    pub context:&'a mut ExecContext,
    pub modules:&'a mut EvalModules
}

#[derive(Finalize,Trace)]
pub enum Function {
    NativeFn(#[unsafe_ignore_trace] fn(&mut ExecScope,args:Vec<Variable>) -> Variable),
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

impl<'a> ExecScope<'a> {
    pub fn find_userdata<T>(&self,name:&str) -> Option<&mut T> {
        let user_var = self.context.find_symbol(None, name, &self.modules)?;
        let ptr = user_var.cast_userdata()?;
        let value_ptr = unsafe { &mut *(ptr as *mut T) };
        Some(value_ptr)
    }
}