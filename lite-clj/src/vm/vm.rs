use crate::vm::call_frame::{CallFrame};
use crate::vm::value::value::{Value};
pub struct VM {
    pub stack: Vec<Value>,
    pub frames: Vec<CallFrame>,
}

impl VM {
    pub fn new(stack_size:usize) -> VM {
        VM {
            stack:Vec::with_capacity(stack_size),
            frames:vec![]
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new(4096)
    }
}


#[test]
fn test_vm() {
    let mut vm = VM::default();
    
    
}