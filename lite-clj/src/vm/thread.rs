use crate::vm::stack::Stack;
use crate::vm::gc::{GC,GcPtr,Generation,Move,CloneUnrooted,Borrow};
use crate::vm::vm::GlobalVmState;
use core::f64;
use std::{ ops::{Deref, DerefMut,Add,Sub,Mul,Div}, sync::{Arc,Mutex,MutexGuard}};
use crate::vm::stack::{StackFrame,State,ClosureState};
use super::{Getable, ValueRepr,Value, errors::Error, instruction::Instruction, stack::StackState, value::{ClosureData}};
pub struct RootedThread {
    thread: GcPtr<Thread>,
    rooted: bool
}

impl Deref for RootedThread {
    type Target = Thread;
    fn deref(&self) -> &Thread {
        &self.thread
    }
}

impl RootedThread {
    pub fn new() -> RootedThread {
        Self::with_global_state(GlobalVmState::default())
    }

    pub fn with_global_state(mut global_state: GlobalVmState) -> RootedThread {
        let context = Mutex::new(Context::new(
            global_state.gc.get_mut().unwrap().new_child_gc(),
        ));
        let global_state = Arc::new(global_state);
        let thread = Thread {
            global_state: global_state.clone(),
            context,
        };
        let ptr = unsafe {
            let mut gc = GC::new(Generation::default(), usize::MAX);
            let ptr = gc.alloc_owned(Move(thread)).unrooted();
            *ptr.global_state.gc.lock().unwrap() = gc;
            let ptr = GcPtr::from(ptr);
            ptr
        };
        let vm = ptr.root_thread();
        {
            let mut context = vm.context.lock().unwrap();
            StackFrame::<State>::new_frame(&mut context.stack, 0, State::Unknown).unwrap();
        }
        vm
    }
}

pub struct Thread {
    context: Mutex<Context>,
    pub global_state: Arc<GlobalVmState>,
}

impl Thread {
    pub fn root_thread(&self) -> RootedThread {
        unsafe {
            let thread = GcPtr::from_raw(self);
            RootedThread {
                thread,
                rooted: true,
            }
        }
    }

    fn context(&self) -> OwnedContext {
        OwnedContext {
            thread: self,
            context: self.context.lock().unwrap(),
        }
    }

    pub fn call_thunk(&self, closure: &GcPtr<ClosureData>) -> Result<Value,Error> {
        let mut context = self.context();
        context.stack.push(crate::construct_gc!(ValueRepr::Closure(@&closure)));
        StackFrame::<State>::current(&mut context.stack).enter_scope(
            0,
            &*crate::construct_gc!(ClosureState {
                @closure: Borrow::new(closure),
                instruction_index: 0,
            }),
        ).unwrap();
        let mut context = match context.execute() {
            Ok(ctx) => ctx.expect("call_module to have the stack remaining"),
            Err(err) => return  Err(err)
        };
        let last_value = context.stack.last().unwrap();
        let clone_value = unsafe { last_value.get_value().clone_unrooted() };
        context.stack.pop();
        Ok(clone_value)
    }
}

pub struct Context {  
    pub(crate) stack: Stack,
    pub(crate) gc: GC,
}

impl Context {
    fn new(gc: GC) -> Context {
        Context {
            stack:Stack::new(),
            gc
        }
    }

    
}

pub struct OwnedContext<'b> {
    thread: &'b Thread,
    context: MutexGuard<'b, Context>,
}

impl<'b> Deref for OwnedContext<'b> {
    type Target = Context;
    fn deref(&self) -> &Context {
        &self.context
    }
}

impl<'b> DerefMut for OwnedContext<'b> {
    fn deref_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl <'b> OwnedContext<'b> {
    fn execute(mut self) -> Result<Option<OwnedContext<'b>>,Error> {
        let mut context = self.borrow_mut();
        let state:&State = &context.stack_frame.frame().state;
        loop {
            let state:&State = &context.stack_frame.frame().state;
            match  state {
                State::Unknown => {
                    return Ok(Some(self)).into();
                }
                State::Closure(ClosureState {
                    closure,
                    instruction_index,
                }) => {
                    let closure_context = context.from_state();
                    match closure_context.execute_()? {
                        Some(new_context) =>  {
                            context = new_context;  
                        },
                        None => return Ok(None)
                    }
                },
            }
        }
    }

    fn borrow_mut(&mut self) -> ExecuteContext<State> {
        let thread = self.thread;
        let context = &mut **self;
        ExecuteContext {
            thread,
            gc: &mut context.gc,
            stack_frame: StackFrame::<State>::current(&mut context.stack),
        }
    }
}

pub struct ExecuteContext<'b, 'gc, S: StackState = ClosureState> {
    pub thread: &'b Thread,
    pub stack_frame: StackFrame<'b,S>,
    pub gc: &'gc mut GC
}

impl<'b, 'gc> ExecuteContext<'b, 'gc>  {
    fn execute_(mut self) -> Result<Option<ExecuteContext<'b,'gc,State>>,Error> {
        println!("enter execute_");
        let state = &self.stack_frame.frame().state;
        let function = unsafe { state.closure.function.clone_unrooted() };
        let instructions = &function.instructions[..];
        let mut program_counter = ProgramCounter::new(state.instruction_index, instructions);
        loop {
            let instr = unsafe { program_counter.instruction() };
            println!("instr:{:?}",&instr);
            let instruction_index = program_counter.instruction_index;
            program_counter.step();

            match  instr {
                Instruction::PushInt(i) => self.stack_frame.push(ValueRepr::Int(i)),
                Instruction::AddInt =>  binop_int(self.thread, &mut self.stack_frame, i64::checked_add)?,
                Instruction::SubtractInt => binop_int(self.thread, &mut self.stack_frame, i64::checked_sub)?,
                Instruction::DivideInt => binop_int(self.thread, &mut self.stack_frame, i64::checked_div)?,
                Instruction::MultiplyInt => binop_int(self.thread, &mut self.stack_frame, i64::checked_mul)?,
                Instruction::IntLT => binop_bool(self.thread, &mut self.stack_frame, |l: i64, r| l < r)?,
                Instruction::IntEQ => binop_bool(self.thread, &mut self.stack_frame, |l: i64, r| l == r)?,

                Instruction::PushByte(num) => self.stack_frame.push(ValueRepr::Byte(num)),
                Instruction::AddByte =>  binop_byte(self.thread, &mut self.stack_frame, u8::checked_add)?,
                Instruction::SubtractByte => binop_byte(self.thread, &mut self.stack_frame, u8::checked_sub)?,
                Instruction::DivideByte => binop_byte(self.thread, &mut self.stack_frame, u8::checked_div)?,
                Instruction::MultiplyByte => binop_byte(self.thread, &mut self.stack_frame, u8::checked_mul)?,
                Instruction::ByteLT => binop_bool(self.thread, &mut self.stack_frame, |l: u8, r| l < r)?,
                Instruction::ByteEQ => binop_bool(self.thread, &mut self.stack_frame, |l: u8, r| l == r)?,
                
                Instruction::PushFloat(eqfloat) => self.stack_frame.push(ValueRepr::Float(eqfloat.into())),
                Instruction::AddFloat =>  binop_f64(self.thread, &mut self.stack_frame, f64::add)?,
                Instruction::SubtractFloat => binop_f64(self.thread, &mut self.stack_frame, f64::sub)?,
                Instruction::DivideFloat => binop_f64(self.thread, &mut self.stack_frame, f64::div)?,
                Instruction::MultiplyFloat => binop_f64(self.thread, &mut self.stack_frame, f64::mul)?,
                Instruction::FloatLT => binop_bool(self.thread, &mut self.stack_frame, |l: f64, r| l < r)?,
                Instruction::FloatEQ => binop_bool(self.thread, &mut self.stack_frame, |l: f64, r| l == r)?,
                Instruction::Return => {
                    drop(program_counter);
                    break;
                },
            
                Instruction::NewClosure { function_index, upvars } => {}
                Instruction::CloseClosure(idx) => {}
                Instruction::Call(_) => {}
                Instruction::TailCall(_) => {}
                Instruction::Push(_) => {}
            }
        };
        let len = self.stack_frame.len();
        let (stack_exists, mut context) = {
            let r = self.exit_scope();
            (
                r.is_ok(),
                match r {
                    Ok(context) => context,
                    Err(context) => context,
                },
            )
        };
        
        
        context.stack_frame.slide(len);
        println!("execute_ end stack {:?}",context.stack_frame.stack.values);
        Ok(if stack_exists { Some(context) } else { None })
    }

    fn exit_scope(self) -> Result<ExecuteContext<'b, 'gc, State>,ExecuteContext<'b, 'gc, State>> {
        match self.stack_frame.exit_scope() {
            Ok(stack) => {
                Ok(ExecuteContext {
                    thread: self.thread,
                    stack_frame:stack,
                    gc: self.gc,
                })
            }
            Err(stack) => {
                Err(ExecuteContext {
                    thread:self.thread,
                    stack_frame:StackFrame::<State>::current(stack),
                    gc:self.gc
                })
            }
        }
    }

}

impl<'b, 'gc> ExecuteContext<'b, 'gc, State> {
    fn from_state<T>(self) -> ExecuteContext<'b, 'gc, T>
    where
        T: StackState,
    {
        ExecuteContext {
            thread: self.thread,
            stack_frame: self.stack_frame.from_state(),
            gc: self.gc
        }
    }
}
struct ProgramCounter<'a> {
    instruction_index: usize,
    instructions: &'a [Instruction],
}

impl<'a> ProgramCounter<'a> {
    fn new(instruction_index: usize, instructions: &'a [Instruction]) -> Self {
        assert!(instruction_index < instructions.len());
        assert!(instructions.last() == Some(&Instruction::Return));
        ProgramCounter {
            instruction_index,
            instructions,
        }
    }

    #[inline(always)]
    unsafe fn instruction(&self) -> Instruction {
        *self.instructions.get_unchecked(self.instruction_index)
    }

    #[inline(always)]
    fn step(&mut self) {
        self.instruction_index += 1;
    }

    #[inline(always)]
    fn jump(&mut self, index: usize) {
        assert!(index < self.instructions.len());
        self.instruction_index = index;
    }
}


#[inline(always)]
fn binop<'b, 'c, F, T>(vm: &'b Thread,stack: &'b mut StackFrame<'c, ClosureState>,f: F) -> Result<(),Error> 
  where F: FnOnce(T, T) -> Result<ValueRepr,Error>,T: for<'d, 'e> Getable<'d, 'e>  {
    assert!(stack.len() >= 2);
    let r = stack.get_value(vm, stack.len() - 1).unwrap();
    let l = stack.get_value(vm, stack.len() - 2).unwrap();
    let result = f(l, r)?;
    stack.pop();
    *stack.last_mut().unwrap() = result.into();
    Ok(())
}

#[inline(always)]
fn binop_int<'b, 'c, F, T>(vm: &'b Thread,stack: &'b mut StackFrame<'c, ClosureState>,f: F) -> Result<(),Error> 
  where F: FnOnce(T, T) -> Option<i64>,T: for<'d, 'e> Getable<'d, 'e> {
    binop(vm, stack, |l, r| {
        Ok(ValueRepr::Int(f(l, r).ok_or_else(|| {
            Error::Message("Arithmetic overflow".into())
        })?))
    })
}

#[inline(always)]
fn binop_byte<'b, 'c, F, T>(vm: &'b Thread,stack: &'b mut StackFrame<'c, ClosureState>,f: F) -> Result<(),Error> 
 where F: FnOnce(T, T) -> Option<u8>, T: for<'d, 'e> Getable<'d, 'e> {
    binop(vm, stack, |l, r| {
        Ok(ValueRepr::Byte(f(l, r).ok_or_else(|| {
            Error::Message("Arithmetic overflow".into())
        })?))
    })
}

#[inline(always)]
fn binop_bool<'b, 'c, F, T>(vm: &'b Thread,stack: &'b mut StackFrame<'c, ClosureState>,f: F) -> Result<(),Error>
 where
    F: FnOnce(T, T) -> bool,
    T: for<'d, 'e> Getable<'d, 'e> {
    binop(vm, stack, |l, r| {
        Ok(ValueRepr::Tag(if f(l, r) { 1 } else { 0 }))
    })
}

#[inline(always)]
fn binop_f64<'b, 'c, F, T>(vm: &'b Thread,stack: &'b mut StackFrame<'c, ClosureState>,f: F) -> Result<(),Error>
 where
    F: FnOnce(T, T) -> f64,
    T: for<'d, 'e> Getable<'d, 'e> {
    binop(vm, stack, |l, r| Ok(ValueRepr::Float(f(l, r))))
}
