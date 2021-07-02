use std::sync::Mutex;
use crate::gc::{GC,Generation};
pub struct GlobalVmState {
    pub gc: Mutex<GC>
}

impl Default for GlobalVmState {
    fn default() -> Self {
        GlobalVmState {
            gc:Mutex::new(GC::new(Generation::default(), usize::MAX))
        }
    }
}