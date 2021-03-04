use super::Value;
use super::gc::gc::GcPtr;
use super::value::ClosureData;
pub struct Stack {
    values:Vec<Value>,
    frames: Vec<FrameState>,
    max_stack_size:u32
}

pub struct ClosureState {
    pub(crate) closure: GcPtr<ClosureData>,
    pub(crate) instruction_index: usize,
}

pub enum State {
    Unknown,
    Closure(#[cfg_attr(feature = "serde_derive", serde(state))] ClosureState)
}


pub struct FrameState {
    pub offset:u32,
    state:State
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            values:Vec::new(),
            frames:Vec::new(),
            max_stack_size:u32::MAX
        }
    }

    pub fn set_max_stack_size(&mut self,max_stack_size:u32) {
        self.max_stack_size = max_stack_size
    }
}

pub struct StackFrame<'b> {
    stack: &'b mut Stack,
    frame: FrameState,
}

impl<'b> StackFrame<'b> {
    pub fn new_frame(stack: &'b mut Stack, args: u32, state: State) -> StackFrame<'b> {
        todo!()
    }
}

mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
    }

}