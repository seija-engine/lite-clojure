pub mod thread;
#[macro_use]
pub mod gc;
#[macro_use]
pub mod stack;
pub mod value;
mod errors;
pub mod vm;
mod getable;

pub mod instruction;

pub use {
    value::{Value,ValueRepr},
    stack::{},
    thread::{Thread},
    value::Variants,
    errors::Error,
    value::BytecodeFunction,
    instruction::{Instruction}
};


pub trait Getable<'vm, 'value>: Sized {

    fn from_value(vm: &'vm thread::Thread, value: value::Variants<'value>) -> Self;
}