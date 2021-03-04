use crate::vm::gc::gc::{GcPtr,CloneUnrooted,CopyUnrooted};
use crate::vm::instruction::{Instruction};
pub struct Value(ValueRepr);

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

pub enum ValueRepr {
    Byte(u8),
    Int(i64),
    Float(f64),
    Closure(GcPtr<ClosureData>),
}

pub struct ClosureData {
    pub function: GcPtr<BytecodeFunction>,
    pub(crate) upvars: Vec<Value>,
}

pub struct BytecodeFunction {
    pub args:u32,
    pub max_stack_size: u32,
    pub instructions: Vec<Instruction>,
    pub inner_functions: Vec<GcPtr<BytecodeFunction>>,
}