use super::value::EqFloat;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum Instruction {
    PushNil,
    PushInt(i64),
    PushByte(u8),
    PushFloat(EqFloat),
    Push(u32),

    NewClosure {
        function_index: u32,
        upvars: u32,
    },
    CloseClosure(u32),

    Call(u32),
    TailCall(u32),
    AddByte,
    SubtractByte,
    MultiplyByte,
    DivideByte,
    ByteLT,
    ByteEQ,
    
    AddInt,
    SubtractInt,
    MultiplyInt,
    DivideInt,
    IntLT,
    IntEQ,

    AddFloat,
    SubtractFloat,
    MultiplyFloat,
    DivideFloat,
    FloatLT,
    FloatEQ,

    Return,
}