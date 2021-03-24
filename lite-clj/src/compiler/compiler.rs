use crate::{ast::{ast::ASTModule, expr::Expr}, vm::Instruction};

#[derive(Debug)]
pub struct CompiledFunction {
    pub args: u32,

    pub instructions: Vec<Instruction>,
    pub inner_functions: Vec<CompiledFunction>,
}

#[derive(Debug)]
pub struct CompiledModule {
    pub function: CompiledFunction,
}

impl From<CompiledFunction> for CompiledModule {
    fn from(function: CompiledFunction) -> Self {
        CompiledModule {
            function
        }
    }
}

pub struct Compiler {

}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile_ast_module(&mut self,ast_module:ASTModule) -> Result<CompiledModule,()> {
        for expr in ast_module.exprs {
            self.compile_expr(expr)
        }
        todo!()
    }

    pub fn compile_expr(&mut self,expr:Expr) {
        
    }
}