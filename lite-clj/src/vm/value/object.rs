use crate::vm::value::function::{Function,Closure};
pub enum Object {
    String(String),
    Function(Function),
    Closure(Closure),
}

impl Object {
    pub fn cast_string(&self) -> Option<&String> {
        if let Object::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_closure(&self) -> Option<&Closure> {
        if let Object::Closure(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_closure_mut(&mut self) -> Option<&mut Closure> {
        if let Object::Closure(ref mut o) = *self {
            Some(o)
        } else {
            None
        }
    }

    pub fn as_function(&self) -> Option<&Function> {
        if let Object::Function(v) = self {
            Some(v)
        } else {
            None
        }
    }
}