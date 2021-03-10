use crate::vm::gc::gc::{GcPtr,CloneUnrooted,CopyUnrooted};
use crate::vm::instruction::{Instruction};
use std::mem;

#[repr(transparent)]
#[derive(Debug)]
pub struct Value(ValueRepr);

impl Value {
    pub(crate) fn from_ref(v: &ValueRepr) -> &Value {
        unsafe { mem::transmute(v) }
    }
}

unsafe impl CopyUnrooted for Value {}
impl CloneUnrooted for Value {
    type Value = Self;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self {
        self.copy_unrooted()
    }
}

impl From<ValueRepr> for Value {
    #[inline]
    fn from(x: ValueRepr) -> Value {
        Value(x)
    }
}
#[derive(Debug)]
pub enum ValueRepr {
    Byte(u8),
    Int(i64),
    Float(f64),
    Closure(GcPtr<ClosureData>),
}
#[derive(Debug)]
pub struct ClosureData {
    pub function: GcPtr<BytecodeFunction>,
    pub(crate) upvars: Vec<Value>,
}
#[derive(Debug)]
pub struct BytecodeFunction {
    pub args:u32,
    pub max_stack_size: u32,
    pub instructions: Vec<Instruction>,
    pub inner_functions: Vec<GcPtr<BytecodeFunction>>,
}