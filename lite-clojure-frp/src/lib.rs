use std::collections::HashMap;
use fns::add_frp_fns;
use lite_clojure_eval::{Variable, EvalRT, EvalError};
pub mod fns;
pub type EventID = u32;

pub struct Event {
    pub id:EventID,
    pub f:Option<Variable>,
    pub next_events:Vec<EventID>,
}

pub struct Behavior {
    pub value:Variable
}

#[derive(Default)]
pub struct FRPSystem {
    auto_id:u32,
    events:HashMap<u32,Event>,
}

impl FRPSystem {
    pub fn new_event(&mut self,var:Option<Variable>) -> EventID {
       let ev =  Event { id: self.auto_id,f:var, next_events:vec![] };
        self.events.insert(self.auto_id, ev);
        self.auto_id += 1;
        self.auto_id - 1
    }

    pub fn new_next_event(&mut self,event_id:u32,var:Variable) -> Option<EventID> {
        let new_event = self.new_event(Some(var));
        let pre_ev = self.events.get_mut(&event_id)?;
        pre_ev.next_events.push(new_event);
        Some(new_event)
    }

    pub fn fire(&self,eid:EventID,var:&Variable,vm:&mut EvalRT) -> Result<(),EvalError> {
        if let Some(e) = self.events.get(&eid) {
            let eval_var = if let Some(event_var) = e.f.as_ref() {
                match event_var {
                    Variable::Function(_) => {
                       vm.invoke_func2(&event_var, vec![var.clone()])?
                    },
                    other => { other.clone() }
                }
            } else {
                var.clone()
            };
            println!("{:?}",&eval_var);
            for child_event_id in e.next_events.iter() {
                self.fire(*child_event_id, &eval_var, vm)?;    
            }
        }
        Ok(())
    }
}

#[test]
fn testt() {
    let mut vm = EvalRT::new();
    vm.init();
    add_frp_fns(&mut vm);
    let mut system = FRPSystem::default();
    let e0 = system.new_event(None);
    vm.global_context().push_var("eRoot", Variable::Int(e0 as i64));

    let frp_ptr = &mut system as *mut FRPSystem as *mut u8;
    vm.global_context().push_var("*FRPSYSTEM*", Variable::UserData(frp_ptr));

    vm.eval_file("tests/1.clj");
}