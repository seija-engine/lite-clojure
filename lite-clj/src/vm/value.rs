use crate::vm::gc::gc::{GcPtr,CloneUnrooted,CopyUnrooted};
use crate::vm::instruction::{Instruction};
use std::{char::MAX, mem};
use std::marker::PhantomData;

#[derive(Debug)]
#[repr(transparent)]
pub struct Variants<'a>(pub ValueRepr, PhantomData<&'a Value>);

impl<'a> Variants<'a> {
    #[inline]
    pub fn new(value: &Value) -> Variants {
        unsafe { Variants::with_root(value, value) }
    }

    #[inline]
    pub(crate) unsafe fn with_root<'r, T: ?Sized>(value: &Value, _root: &'r T) -> Variants<'r> {
        Variants(value.get_repr().clone_unrooted(), PhantomData)
    }

    #[inline]
    pub fn get_value(&self) -> &Value {
        Value::from_ref(&self.0)
    }
}

#[repr(transparent)]
#[derive(Debug,PartialEq)]
pub struct Value(pub ValueRepr);

impl Value {
    pub(crate) fn from_ref(v: &ValueRepr) -> &Value {
        unsafe { mem::transmute(v) }
    }

    pub(crate) fn get_repr(&self) -> &ValueRepr {
        &self.0
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
#[derive(Debug,PartialEq)]
pub enum ValueRepr {
    Byte(u8),
    Int(i64),
    Float(f64),
    Tag(u32),
    Closure(GcPtr<ClosureData>),
}

unsafe impl CopyUnrooted for ValueRepr {}

impl CloneUnrooted for ValueRepr {
    type Value = Self;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self {
        self.copy_unrooted()
    }
}

#[derive(Debug,PartialEq)]
pub struct ClosureData {
    pub function: GcPtr<BytecodeFunction>,
    pub upvars: Vec<Value>,
}
#[derive(Debug,PartialEq)]
pub struct BytecodeFunction {
    pub args:u32,
    pub max_stack_size: u32,
    pub instructions: Vec<Instruction>,
    pub inner_functions: Vec<GcPtr<BytecodeFunction>>,
}

impl Default for BytecodeFunction {
    fn default() -> Self {
        BytecodeFunction {
            args: 0,
            max_stack_size:u32::MAX,
            instructions:Vec::new(),
            inner_functions:Vec::new(),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub struct EqFloat(pub f64);

impl From<f64> for EqFloat {
    fn from(f: f64) -> Self {
        EqFloat(f)
    }
}

impl From<EqFloat> for f64 {
    fn from(f: EqFloat) -> Self {
        f.0
    }
}

impl EqFloat {
    fn key(&self) -> u64 {
        unsafe { std::mem::transmute(self.0) }
    }
}

impl Eq for EqFloat {}

impl PartialEq for EqFloat {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl std::hash::Hash for EqFloat {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.key().hash(hasher)
    }
}