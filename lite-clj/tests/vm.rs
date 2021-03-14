use std::thread;
use Instruction::*;
use lite_clj::vm::{BytecodeFunction, Instruction, Value, ValueRepr, gc::{GC, GcPtr, Move}, thread::{RootedThread}, value::ClosureData};

fn test_byte_code(instrs:Vec<Instruction>) -> Value {
    let vm= RootedThread::new();
    let mut cur_gc = vm.global_state.gc.lock().unwrap();
    let code_ptr = new_byte_code_func(&mut cur_gc,instrs,0);
    let closure_data = ClosureData {
        function:code_ptr,
        upvars:vec![]
     };
     let closure_ptr = unsafe { cur_gc.alloc(Move(closure_data)).unrooted() };
     let call_value = vm.call_thunk(&closure_ptr).unwrap();
     call_value
}

fn new_byte_code_func(gc:&mut GC,instrs:Vec<Instruction>,args:u32) -> GcPtr<BytecodeFunction> {
    let mut byte_code = BytecodeFunction::default();
    byte_code.instructions = instrs;
    byte_code.args = args;
    unsafe { gc.alloc(Move(byte_code)).unrooted() }
}

#[test]
fn test_vm_int() {
    let ret_value = test_byte_code(vec![PushInt(2),PushInt(4),MultiplyInt,Return]);
    assert_eq!(ret_value.0,ValueRepr::Int(8));
}

#[test]
fn test_vm_float() {
    
    let bytes = vec![PushFloat(10.0.into()),
                                    PushFloat(3.0.into()),
                                    DivideFloat,
                                    PushFloat(2.0.into()),
                                    MultiplyFloat,
                                    PushFloat((10.0 / 3.0 * 2.0).into()),
                                    FloatEQ,
                                    Return
                                ];
    let ret_value = test_byte_code(bytes);
    assert_eq!(ret_value.0,ValueRepr::Tag(1));
}

#[test]
fn test_call_func() {
    let vm= RootedThread::new();
    let mut cur_gc = vm.global_state.gc.lock().unwrap();
    let mut main_func = BytecodeFunction::from_instr(vec![
        NewClosure {function_index:0,upvars:0},
        Push(0),
        PushInt(12450),
        PushInt(2),
        Call(2),
        Return
    ]);
    
    let mult_func = new_byte_code_func(&mut cur_gc,vec![
        Push(0),
        Push(1),
        MultiplyInt,
        Return
    ],2);
    main_func.inner_functions = vec![mult_func];
    
    let closure_data = ClosureData {
        function:unsafe { cur_gc.alloc(Move(main_func)).unrooted() },
        upvars:vec![]
     };
     let closure_ptr = unsafe { cur_gc.alloc(Move(closure_data)).unrooted() };
     let call_value = vm.call_thunk(&closure_ptr).unwrap();
     println!("callValue {:?}",call_value);
}