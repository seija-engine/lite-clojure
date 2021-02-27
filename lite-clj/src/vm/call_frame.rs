use crate::vm::value::object::Object;
pub struct CallFrame {
    closure: Object,
    ip: usize,
    stack_start: usize,
}

impl CallFrame {
    pub fn new(closure: Object, stack_start: usize) -> Self {
        CallFrame {
            closure,
            ip: 0,
            stack_start,
        }
    }
}