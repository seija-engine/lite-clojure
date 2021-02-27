use crate::vm::code_chunk::CodeChunk;

#[derive(Debug)]
pub struct FunctionBuilder {
    name: String,
    pub chunk: CodeChunk,
    arity: u8,
    upvalue_count: usize,
}

impl FunctionBuilder {
    pub fn new(name: &str, arity: u8) -> Self {
        let name: String = name.into();
        let chunk = CodeChunk::new(name.clone());
        FunctionBuilder { name, arity, chunk, upvalue_count: 0 }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_upvalue_count(&mut self, count: usize) {
        self.upvalue_count = count;
    }

    pub fn build(self) -> Function {
        Function::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    chunk: CodeChunk,
    arity: u8,
    upvalue_count: usize,
}

impl Function {
    fn new(builder: FunctionBuilder) -> Self {
        Function {
            name: builder.name,
            arity: builder.arity,
            chunk: builder.chunk,
            upvalue_count: builder.upvalue_count,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn chunk(&self) -> &CodeChunk {
        &self.chunk
    }

    pub fn upvalue_count(&self) -> usize {
        self.upvalue_count
    }
}

#[derive(Debug, Clone)]
pub struct Closure {
    function: Function,
}

impl Closure {
    pub fn new(function: Function) -> Self {
        Closure {
            function,
        }
    }

    pub fn name(&self) -> &str {
        self.function.name()
    }

    pub fn arity(&self) -> u8 {
        self.function.arity
    }

    pub fn chunk(&self) -> &CodeChunk {
        self.function.chunk()
    }
    
}