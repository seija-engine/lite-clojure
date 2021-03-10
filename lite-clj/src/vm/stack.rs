use super::{Value,ValueRepr};
use super::errors::{Error};
use super::gc::{gc::{GcPtr,Borrow,CloneUnrooted,CopyUnrooted}};
use super::value::ClosureData;
#[derive(Debug)]
pub struct Stack {
    values:Vec<Value>,
    frames: Vec<FrameState>,
    max_stack_size:u32
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

    pub fn len(&self) -> u32 {
        self.values.len() as u32
    }

    pub fn current_frame(&mut self) -> StackFrame {
        let frame: &FrameState = &*self.frames.last().expect("Frame").from_state();
        StackFrame {
            frame: unsafe { frame.clone_unrooted() },
            stack: self,
        }
    }

    pub fn remove_range(&mut self, from: u32, to: u32) {
        
        self.values.drain(from as usize..to as usize);
    }
}
#[derive(Debug)]
pub struct ClosureState {
    pub(crate) closure: GcPtr<ClosureData>,
    pub(crate) instruction_index: usize,
}

impl ClosureState {
    fn max_stack_size(&self) -> u32 {
        self.closure.function.max_stack_size
    }
}
#[derive(Debug)]
pub enum State {
    Unknown,
    Closure(#[cfg_attr(feature = "serde_derive", serde(state))] ClosureState)
}

impl State {
    fn from_state(state: &State) -> &Self {
        state
    }
    fn to_state(&self) -> Borrow<State> {
        Borrow::new(self)
    }

    fn max_stack_size(&self) -> u32 {
        match self {
            State::Unknown => 0,
            State::Closure(closure) =>todo!()
        }
    }
}


unsafe impl CopyUnrooted for State {}
impl CloneUnrooted for State {
    type Value = Self;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self {
        self.copy_unrooted()
    }
}

#[derive(Debug)]
pub struct FrameState {
    pub offset:u32,
    pub state:State
}

impl FrameState {
    fn from_state(&self) -> Borrow<FrameState> {
        crate::construct_gc!(FrameState {
            offset: self.offset,
            @state: Borrow::new(&self.state),
            
        })
    }
}

unsafe impl CopyUnrooted for FrameState {}
impl CloneUnrooted for FrameState {
    type Value = Self;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self {
        self.copy_unrooted()
    }
}



pub struct StackFrame<'b> {
    stack: &'b mut Stack,
    frame: FrameState,
}

impl<'b> StackFrame<'b> {
    pub fn new_frame(stack: &'b mut Stack, args: u32, state: State) -> Result<StackFrame<'b>,Error>  {
        let frame = unsafe { Self::add_new_frame(stack, args, &state, false)?.unrooted() };
        Ok(StackFrame { stack, frame })
    }

    pub(crate) fn enter_scope(self, args: u32, state: &State) -> Result<StackFrame<'b>,Error> {
        self.enter_scope_excess(args, state, false)
    }

    pub(crate) fn enter_scope_excess(self,args: u32,state: &State,excess: bool) -> Result<StackFrame<'b>,Error> {
        let stack = self.stack;
        let frame = unsafe { Self::add_new_frame(stack, args, state, excess)?.unrooted() };
        Ok(StackFrame { stack, frame })
    }

    pub(crate) fn exit_scope(self) -> std::result::Result<StackFrame<'b>, &'b mut Stack> {
        //TODO 外部函数检测？
        let stack = self.stack;
        stack.frames.pop().expect("Expected frame");
        match stack.frames.last() {
            Some(frame) => {
                let stack = StackFrame {
                    frame: unsafe { frame.clone_unrooted() },
                    stack,
                };
                Ok(stack)
            }
            None => Err(stack),
        }
    }

    fn add_new_frame<'gc>(stack: &mut Stack,args: u32,state: &'gc State,excess:bool) -> Result<Borrow<'gc, FrameState>,Error> {
        //确保args已经压入栈内
        assert!(stack.len() >= args);
        let offset = stack.len() - args;
        let frame = crate::construct_gc!(FrameState {
            offset,
            @state: Borrow::new(state),
        });
        if let Some(frame) = stack.frames.last() {
            assert!(frame.offset <= offset);
            //TODO 外部函数参数检测？
        }
        if stack.len() + frame.state.max_stack_size() > stack.max_stack_size {
            return Err(Error::StackOverflow);
        }

        unsafe {
            //用clone_unrooted Clone了一次
            stack.frames.push(frame.clone_unrooted());
        }
        Ok(frame)
    }

    #[inline(always)]
    pub fn push<T>(&mut self, v: T) where T: StackPrimitive {
        v.push_to(&mut self.stack)
    }

    pub(crate) fn current(stack: &mut Stack) -> StackFrame {
        stack.current_frame()
    }

    fn offset(&self) -> u32 {
        self.frame.offset
    }

    pub fn remove_range(&mut self, from: u32, to: u32) {
        let offset = self.offset();
        self.stack.remove_range(offset + from, offset + to);
    }

    pub fn insert_slice(&mut self, index: u32, values: &[Value]) {
        let index = (self.offset() + index) as usize;
        unsafe {
            self.stack.values.splice(index..index, values.iter().map(|v| v.clone_unrooted()));
        }
    }
}

pub trait StackPrimitive {
    fn push_to(&self, stack: &mut Stack);

    fn extend_to<'b, I>(iter: I, stack: &mut Stack) where I: IntoIterator<Item = &'b Self>, Self: 'b {
        for item in iter {
            item.push_to(stack);
        }
    }
}

impl StackPrimitive for Value {
    #[inline(always)]
    fn push_to(&self, stack: &mut Stack) {
        unsafe {
            stack.values.push(self.clone_unrooted());
        }
    }

    #[inline(always)]
    fn extend_to<'b, I>(iter: I, stack: &mut Stack)  where I: IntoIterator<Item = &'b Self> {
        unsafe {
            stack.values.extend(iter.into_iter().map(|i| i.clone_unrooted()));
        }
    }
}

impl StackPrimitive for ValueRepr {
    #[inline(always)]
    fn push_to(&self, stack: &mut Stack) {
        Value::from_ref(self).push_to(stack)
    }

    fn extend_to<'b, I>(iter: I, stack: &mut Stack)
    where
        I: IntoIterator<Item = &'b Self>,
    {
        Value::extend_to(iter.into_iter().map(Value::from_ref), stack)
    }
}

mod tests {
    use super::{Stack, StackFrame, State};
    use crate::vm::value::{ValueRepr,Value};
    #[test]
    fn test_stack() {
       
        let mut stack = Stack::new();
        let mut frame = StackFrame::new_frame(&mut stack, 0, State::Unknown).unwrap();
        frame.push(ValueRepr::Int(1));
        frame.push(ValueRepr::Int(2));
        frame = frame.enter_scope(2, &State::Unknown).unwrap();
        frame.push(ValueRepr::Int(2));
        frame.push(ValueRepr::Int(3));
        frame = frame.enter_scope(2, &State::Unknown).unwrap();
        frame.push(ValueRepr::Int(4));
        frame.push(ValueRepr::Int(5));
        frame.push(ValueRepr::Int(6));

        
        frame.remove_range(2, 5);
        let a = 5;
        dbg!(stack);
    }

}