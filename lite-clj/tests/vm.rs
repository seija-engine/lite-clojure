use lite_clj::vm::{BytecodeFunction, Instruction, Value, ValueRepr, gc::{Move}, thread::{RootedThread}, value::ClosureData};

fn test_byte_code(instrs:Vec<Instruction>) -> Value {
    let vm= RootedThread::new();
    let mut byte_code = BytecodeFunction::default();
    byte_code.instructions = instrs;
    let mut cur_gc = vm.global_state.gc.lock().unwrap();
    let code_ptr = unsafe { cur_gc.alloc(Move(byte_code)).unrooted() };
    let closure_data = ClosureData {
        function:code_ptr,
        upvars:vec![]
     };
     let closure_ptr = unsafe { cur_gc.alloc(Move(closure_data)).unrooted() };
     let call_value = vm.call_thunk(&closure_ptr).unwrap();
     call_value
}
#[test]
fn test_vm_int() {
    use Instruction::*;
    let ret_value = test_byte_code(vec![PushInt(2),PushInt(4),MultiplyInt,Return]);
    assert_eq!(ret_value.0,ValueRepr::Int(8));
}

#[test]
fn test_vm_float() {
    use Instruction::*;
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