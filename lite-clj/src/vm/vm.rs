use crate::vm::call_frame::{CallFrame};
pub struct VM {
    //pub stack: Vec<Value>,
    pub frames: Vec<CallFrame>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            frames:vec![]
        }
    }
}
