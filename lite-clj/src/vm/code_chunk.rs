use crate::vm::opcodes;

use super::opcodes::Opcode;
#[derive(Debug, Clone)]
pub struct CodeChunk {
    name:String,
    op_codes:Vec<Opcode>,
}

impl CodeChunk {
    pub fn new(name:String) -> CodeChunk {
        CodeChunk {
            name,
            op_codes:vec![]
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn write(&mut self, op: Opcode, line: usize) {
       
    }
}

