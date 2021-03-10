use crate::vm::stack::Stack;
use crate::vm::gc::gc::GC;
pub struct Thread {
    
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