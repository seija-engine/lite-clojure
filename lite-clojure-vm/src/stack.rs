use std::ops::{Deref, DerefMut, Index, IndexMut};

use super::{Getable, Value, ValueRepr, value::Variants};
use super::errors::{Error};
use super::gc::{gc::{GcPtr,Borrow,CloneUnrooted,CopyUnrooted}};
use super::value::ClosureData;
#[derive(Debug)]
pub struct Stack {
    pub values:Vec<Value>,
    frames: Vec<Frame>,
    max_stack_size:u32
}

impl Index<u32> for Stack {
    type Output = Value;
    fn index(&self, index: u32) -> &Value {
        &self.values[index as usize]
    }
}

impl IndexMut<u32> for Stack {
    fn index_mut(&mut self, index: u32) -> &mut Value {
        &mut self.values[index as usize]
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            values:Vec::new(),
            frames:Vec::new(),
            max_stack_size:u32::MAX
        }
    }

    #[inline(always)]
    pub fn push<T>(&mut self, v: T) where T: StackPrimitive {
        v.push_to(self)
    }

    pub fn set_max_stack_size(&mut self,max_stack_size:u32) {
        self.max_stack_size = max_stack_size
    }

    pub fn len(&self) -> u32 {
        self.values.len() as u32
    }

    pub fn slide(&mut self, count: u32) {
        let last = self.len() - 1;
        let i = last - count;
        self.copy_value(last, i);
        self.pop_many(count);
    }

    pub fn pop_many(&mut self, count: u32) {
        let len = self.values.len();
        self.values.truncate(len - count as usize);
    }

    fn copy_value(&mut self, from: u32, to: u32) {
        unsafe {
            self[to] = self[from].clone_unrooted();
        }
    }

    pub fn current_frame(&mut self) -> StackFrame {
        let frame: &Frame = &*self.frames.last().expect("Frame").from_state();
        StackFrame {
            frame: unsafe { frame.clone_unrooted() },
            stack: self,
        }
    }

    pub fn remove_range(&mut self, from: u32, to: u32) {
        self.values.drain(from as usize..to as usize);
    }

    #[inline]
    pub fn get_variant(&self, index: u32) -> Option<Variants> {
        if index < self.len() {
            Some(Variants::new(&self.values[index as usize]))
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<Variants> {
        self.get_variant(self.len() - 1)
    }

    pub fn pop(&mut self) -> Value {
        self.values.pop().expect("pop on empty stack")
    }

    
}
#[derive(Debug)]
pub struct ClosureState {
    pub(crate) closure: GcPtr<ClosureData>,
    pub(crate) instruction_index: usize,
}

unsafe impl CopyUnrooted for ClosureState {}
impl CloneUnrooted for ClosureState {
    type Value = Self;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self {
        self.copy_unrooted()
    }
}

impl StackState for ClosureState {
    fn from_state(state: &State) -> &Self {
        match state {
            State::Closure(state) => state,
            _ => panic!("Expected closure state, got {:?}", state),
        }
    }
    fn from_state_mut(state: &mut State) -> &mut Self {
        match state {
            State::Closure(state) => state,
            _ => panic!("Expected closure state, got {:?}", state),
        }
    }
    fn to_state(&self) -> Borrow<State> {
        crate::construct_gc!(State::Closure(@ Borrow::new(self)))
    }
    fn max_stack_size(&self) -> u32 {
        self.closure.function.max_stack_size
    }
}

impl ClosureState {
    fn max_stack_size(&self) -> u32 {
        self.closure.function.max_stack_size
    }
}
#[derive(Debug)]
pub enum State {
    Unknown,
    Closure(ClosureState)
}

impl State {
    fn from_state(state: &State) -> &Self {
        state
    }
    fn to_state(&self) -> Borrow<State> {
        Borrow::new(self)
    }
    
    pub fn closure_ref(&self) -> &ClosureState {
        match self {
            State::Unknown => panic!(),
            State::Closure(closure) => closure
        }
    }

    fn max_stack_size(&self) -> u32 {
        match self {
            State::Unknown => 0,
            State::Closure(closure) =>todo!()
        }
    }
}

impl StackState for State {
    fn from_state(state: &State) -> &Self {
        state
    }
    fn from_state_mut(state: &mut State) -> &mut Self {
        state
    }
    fn to_state(&self) -> Borrow<State> {
        Borrow::new(self)
    }

    fn max_stack_size(&self) -> u32 {
        match self {
            State::Unknown => 0,
            State::Closure(closure) => closure.max_stack_size()
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
pub struct Frame<S = State> {
    pub offset:u32, 
    pub state:S
}

impl<S> Frame<S> {
    fn to_state(&self) -> Borrow<Frame<State>> where S: StackState {
        crate::construct_gc!(Frame {
            offset: self.offset,
            @state: self.state.to_state(),
        })
    }
}

impl Frame<State> {
    fn from_state<S>(&self) -> Borrow<Frame<S>>
    where
        S: StackState,
    {
        crate::construct_gc!(Frame {
            offset: self.offset,
            @state:Borrow::new(S::from_state(&self.state)),
        })
    }
}

unsafe impl<S> CopyUnrooted for Frame<S> where S: CopyUnrooted {}
impl<S> CloneUnrooted for Frame<S>
where
    S: CopyUnrooted,
{
    type Value = Self;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self {
        self.copy_unrooted()
    }
}

pub trait StackState: CopyUnrooted + Sized {
    fn from_state(state: &State) -> &Self;
    fn from_state_mut(state: &mut State) -> &mut Self;
    fn to_state(&self) -> Borrow<State>;
    fn max_stack_size(&self) -> u32;
}

#[derive(Debug)]
pub struct StackFrame<'b,S = State> {
    pub stack: &'b mut Stack,
    pub frame: Frame<S>,
}


impl<'a: 'b, 'b> StackFrame<'b, State> {
    pub fn from_state<T>(self) -> StackFrame<'b, T>
    where
        T: StackState,
    {
        let frame = unsafe { Frame::from_state::<T>(self.stack.frames.last().unwrap()).unrooted() };
        StackFrame {
            stack: self.stack,
            frame,
        }
    }

    pub fn new_frame(stack: &'b mut Stack, args: u32, state: State) -> Result<StackFrame<'b>,Error> {
        let frame = unsafe { Self::add_new_frame(stack, args, &state, false)?.unrooted() };
        Ok(StackFrame { stack, frame })
    }

    
}

impl<'b, S> Deref for StackFrame<'b, S>
where
    S: StackState,
{
    type Target = [Value];
    fn deref(&self) -> &[Value] {
        let offset = self.offset();
        &self.stack.values[offset as usize..]
    }
}

impl<'b, S> DerefMut for StackFrame<'b, S>
where
    S: StackState,
{
    fn deref_mut(&mut self) -> &mut [Value] {
        let offset = self.offset();
        &mut self.stack.values[offset as usize..]
    }
}

impl<'b, S> Index<u32> for StackFrame<'b, S>
where
    S: StackState,
{
    type Output = Value;
    fn index(&self, index: u32) -> &Value {
        let offset = self.offset();
        &self.stack.values[(offset + index) as usize]
    }
}
impl<'b, S> IndexMut<u32> for StackFrame<'b, S>
where
    S: StackState,
{
    fn index_mut(&mut self, index: u32) -> &mut Value {
        let offset = self.offset();
        &mut self.stack.values[(offset + index) as usize]
    }
}

impl<'a: 'b, 'b, S> StackFrame<'b, S> where S: StackState {

    pub(crate) fn enter_scope<T>(self, args: u32, state: &T) -> Result<StackFrame<'b,T>,Error> where T:StackState {
        self.enter_scope_excess(args, state, false)
    }

    pub fn to_state(self) -> StackFrame<'b, State> {
        StackFrame {
            stack: self.stack,
            frame: unsafe { self.frame.to_state().unrooted() },
        }
    }

    pub fn frame(&self) -> &Frame<S> {
        &self.frame
    }

    pub fn slide(&mut self, count: u32) {
        self.stack.slide(count);
    }

    pub fn top(&self) -> &Value {
        self.stack.values.last().expect("StackFrame: top")
    }

    pub(crate) fn enter_scope_excess<T>(self,args: u32,state: &T,excess: bool) -> Result<StackFrame<'b,T>,Error> where T:StackState {
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

    fn add_new_frame<'gc,T>(stack: &mut Stack,args: u32,state: &'gc T,excess:bool) -> Result<Borrow<'gc, Frame<T>>,Error> where T:StackState {
        //确保args已经压入栈内
        assert!(stack.len() >= args);
        let offset = stack.len() - args;
        let frame = crate::construct_gc!(Frame {
            offset,
            @state: Borrow::new(state),
        });
        if let Some(frame) = stack.frames.last() {
            assert!(frame.offset <= offset);
            //TODO 外部函数参数检测？
        }
        //if stack.len() + frame.state.max_stack_size() > stack.max_stack_size {
        //    return Err(Error::StackOverflow);
        //}

        unsafe {
            //用clone_unrooted Clone了一次
            stack.frames.push(frame.to_state().clone_unrooted());
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

    pub fn len(&self) -> u32 {
        self.stack.len() - self.offset()
    }

    #[inline]
    pub fn get_variant(&self, index: u32) -> Option<Variants> {
        self.stack.get_variant(self.offset() + index)
    }

    #[inline]
    pub fn get_value<'vm, 'value, T>(&'value self,thread: &'vm crate::thread::Thread,index: u32) -> Option<T> where T:Getable<'vm,'value> {
        self.get_variant(index).map(|v| T::from_value(thread, v))
    }

    pub fn pop(&mut self) -> Value {
      
        self.stack.values.pop().expect("pop on empty stack")
    }
}

impl<'b> StackFrame<'b, ClosureState> {
    pub fn set_instruction_index(&mut self, instruction_index: usize) {
        self.frame.state.instruction_index = instruction_index;
        match self.stack.frames.last_mut() {
            Some(Frame {
                state: State::Closure(closure),
                ..
            }) => closure.instruction_index = instruction_index,
            _ => (),
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

impl<'a, T: StackPrimitive + 'a> StackPrimitive for &'a T {
    #[inline(always)]
    fn push_to(&self, stack: &mut Stack) {
        (**self).push_to(stack)
    }

    fn extend_to<'b, I>(iter: I, stack: &mut Stack)
    where
        I: IntoIterator<Item = &'b Self>,
        Self: 'b,
    {
        StackPrimitive::extend_to(iter.into_iter().map(|i| *i), stack)
    }
}

impl<'a, T: StackPrimitive + 'a> StackPrimitive for Borrow<'a, T> {
    #[inline(always)]
    fn push_to(&self, stack: &mut Stack) {
        (**self).push_to(stack)
    }

    fn extend_to<'b, I>(iter: I, stack: &mut Stack)
    where
        I: IntoIterator<Item = &'b Self>,
        Self: 'b,
    {
        StackPrimitive::extend_to(iter.into_iter().map(|i| &**i), stack)
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

impl<'a> StackPrimitive for Variants<'a> {
    #[inline(always)]
    fn push_to(&self, stack: &mut Stack) {
        self.0.push_to(stack)
    }

    fn extend_to<'b, I>(iter: I, stack: &mut Stack)
    where
        I: IntoIterator<Item = &'b Self>,
        Self: 'b,
    {
        Value::extend_to(iter.into_iter().map(|i| i.get_value()), stack)
    }
}

mod tests {
    use super::{Stack, StackFrame, State};
    use crate::value::{ValueRepr,Value};
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