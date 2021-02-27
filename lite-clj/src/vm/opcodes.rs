#![allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
   Return,
   Constant(u8),
   Print,
   Add,
   Sub,
   Mul,
   Div,

   Not,
   Neg,
}

impl Opcode {
    fn write(&self, buf: &mut Vec<u8>) {
        match *self {
            Opcode::Return => buf.push(0x00),
            Opcode::Constant(idx) => { buf.push(0x01); buf.push(idx); },
            Opcode::Print => buf.push(0x02),
            Opcode::Add => buf.push(0x03),
            Opcode::Sub => buf.push(0x04),
            Opcode::Mul => buf.push(0x05),
            Opcode::Div => buf.push(0x06),
            Opcode::Not => buf.push(0x07),
            Opcode::Neg => buf.push(0x08),
            _ => (),
        }
    }
}