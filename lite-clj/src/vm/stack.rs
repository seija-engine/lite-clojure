use super::Value;
pub struct Stack {
    values:Vec<Value>,
    max_stack_size:u32
}

pub struct Frame {
    pub offset:u32
    
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            values:Vec::new(),
            max_stack_size:u32::MAX
        }
    }

    pub fn set_max_stack_size(&mut self,max_stack_size:u32) {
        self.max_stack_size = max_stack_size
    }
}

