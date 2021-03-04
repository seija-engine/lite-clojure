#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Instruction {
    PushInt(u32),
    PushByte(u8),

    AddInt,
    SubtractInt,
    MultiplyInt,
    DivideInt,
    IntLT,
    IntEQ,

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
}