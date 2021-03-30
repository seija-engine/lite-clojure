use std::ops::{Deref, DerefMut};
use crate::{ast::{ast::ASTModule, cexpr::Number, expr::Expr, value::Symbol}, vm::Instruction};
use super::scoped_map::ScopedMap;

#[derive(Debug)]
pub struct CompiledFunction {
    pub id: Symbol,
    pub args: u32,
    pub max_stack_size: u32,
    pub instructions: Vec<Instruction>,
    pub inner_functions: Vec<CompiledFunction>,
}

impl CompiledFunction {
    pub fn new(args: u32, id: Symbol) -> CompiledFunction {
        CompiledFunction {
            args: args,
            max_stack_size: 0,
            id: id,
            instructions: Vec::new(),
            inner_functions: Vec::new()
        }
    }
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

pub struct FunctionEnv {
    stack: ScopedMap<Symbol, u32>,
    stack_size: u32,
    function: CompiledFunction,
    free_vars: Vec<Symbol>,
    emit_debug_info:bool
}

impl FunctionEnv {
    fn new(
        args: u32,
        id: Symbol,
        _source_name: String,
        emit_debug_info: bool,
    ) -> FunctionEnv {
        FunctionEnv {
            free_vars: Vec::new(),
            stack: ScopedMap::new(),
            stack_size: 0,
            function: CompiledFunction::new(args, id),
            emit_debug_info
        }
    }

    fn emit(&mut self, instruction: Instruction) {
        //todo adjust stack size
        self.function.instructions.push(instruction);
       
    }

}

pub struct FunctionEnvs {
    envs: Vec<FunctionEnv>,
}


impl Deref for FunctionEnvs {
    type Target = FunctionEnv;
    fn deref(&self) -> &FunctionEnv {
        self.envs.last().expect("FunctionEnv")
    }
}

impl DerefMut for FunctionEnvs {
    fn deref_mut(&mut self) -> &mut FunctionEnv {
        self.envs.last_mut().expect("FunctionEnv")
    }
}

impl FunctionEnvs {
    fn new() -> FunctionEnvs {
        FunctionEnvs { envs: vec![] }
    }
}




pub struct Compiler {

}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile_ast_module(&mut self,ast_module:ASTModule) -> Result<CompiledModule,()> {
        let mut env = FunctionEnvs::new();
        for expr in ast_module.exprs {
            self.compile_expr(expr,&mut env)
        }
        Err(())
    }

    pub fn compile_expr(&mut self,expr:Expr, function: &mut FunctionEnvs) {
        match expr {
            Expr::Number(Number::Int(n)) =>  function.emit(Instruction::PushInt(n)),
            Expr::Number(Number::Float(f)) => function.emit(Instruction::PushFloat(f.into())),
            Expr::Def(_,_sym,val) => {
                match val {
                    None => function.emit(Instruction::PushNil),
                    Some(v) => self.compile_expr(*v, function)
                }
            },
            _ => {}
        }
    }
}

#[test]
fn test_compiler() {
   use crate::ast::cst::{ParseCST};
   use crate::ast::ast::TranslateToAST;
   let file_name = "tests/clj/test.clj";
   let code_string = std::fs::read_to_string(file_name).unwrap();
   let mut parser = ParseCST::new(&code_string);
   let cexprs = parser.parse_exprs().unwrap();
   let meta_table = parser.take();

   let mut trans = TranslateToAST::new(file_name.to_string(), cexprs, meta_table);
   let ast_mod = trans.translate();

   let mut compiler = Compiler::new();
   let cm = compiler.compile_ast_module(ast_mod);
}