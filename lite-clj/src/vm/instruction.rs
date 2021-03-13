use super::value::EqFloat;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum Instruction {
    PushInt(i64),
    PushByte(u8),
    PushFloat(EqFloat),

    

    MakeClosure {
        function_index: u32,
        upvars: u32,
    },
    NewClosure {
        function_index: u32,
        upvars: u32,
    },

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