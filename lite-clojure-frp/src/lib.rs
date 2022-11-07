use std::collections::HashMap;
use errors::FRPError;
use lite_clojure_eval::{Variable, EvalRT};
pub mod fns;
pub mod errors;
use anyhow::{anyhow,Result};
pub type EventID = u32;

pub struct Event {
    pub id:EventID,
    pub f:Option<Variable>,

    pub next_events:Vec<EventID>,
    pub next_dynamics:Vec<DynamicID>,
}

pub type DynamicID = u32;

pub struct Dynamic {
    pub value:Variable,
    pub fold_fn:Option<Variable>,
    pub updated:Option<EventID>
}


#[derive(Default)]
pub struct FRPSystem {
    auto_id:u32,
    never_event:EventID,
    pub events:HashMap<u32,Event>,
    pub dynamics:HashMap<u32,Dynamic>
}

impl FRPSystem {
    pub fn new() -> Self {
        let mut system = FRPSystem::default();
        system.never_event = system.new_event(None);
        system
    }

    pub fn never(&self) -> EventID {
        self.never_event
    }

    pub fn new_event(&mut self,var:Option<Variable>) -> EventID {
       let ev =  Event { id: self.auto_id,f:var, next_events:vec![],next_dynamics:vec![] };
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

    pub fn fire(&mut self,event_id:EventID,var:&Variable,vm:&mut EvalRT) -> Result<()> {
        let mut dyn_calls:Vec<(DynamicID,Variable)> = vec![];
        
        self.exec_event(event_id, var, vm,&mut dyn_calls)?;
        
        let mut dyn_events:Vec<(EventID,Variable)> = vec![];
        for (id,var) in dyn_calls.drain(..) {
           let dynamic = self.dynamics.get_mut(&id).ok_or(anyhow!(FRPError::DynamicNotFound))?;
           if let Some(fold_fn) = dynamic.fold_fn.as_ref() {
             dynamic.value = vm.invoke_func2(fold_fn, vec![var.clone(),dynamic.value.clone()]).map_err(FRPError::EvalError)?
           } else {
             dynamic.value = var;
           }
           if let Some(update_id) = dynamic.updated {
                dyn_events.push((update_id,dynamic.value.clone()));
           }
        }
        for (ev_id,dyn_var) in dyn_events.drain(..) {
            self.fire(ev_id, &dyn_var, vm)?;
        }
        Ok(())
    }

    pub fn exec_event(& self,event_id:EventID,var:&Variable,vm:&mut EvalRT,dyn_calls:&mut Vec<(DynamicID,Variable)>) -> Result<()> {
        let cur_event = self.events.get(&event_id).ok_or(anyhow!(FRPError::EventNotFound))?;
        let event_var = if let Some(event_var) = cur_event.f.as_ref() {
            match event_var {
                Variable::Function(_) => {
                   vm.invoke_func2(&event_var, vec![var.clone()]).map_err(FRPError::EvalError)?
                },
                other => { other.clone() }
            }
        } else {
            var.clone()
        };
        for event_id in cur_event.next_events.iter() {
            self.exec_event(*event_id, &event_var, vm,dyn_calls)?;
        }

        for dyn_id in cur_event.next_dynamics.iter() {
            dyn_calls.push((*dyn_id,event_var.clone()));
        }
        Ok(())
    }

    pub fn new_dynamic(&mut self,value:Variable,event:EventID,fold_fn:Option<Variable>) -> Option<DynamicID>  {
        let dynmaic = Dynamic { value,fold_fn,updated:None };
        let event_mut = self.events.get_mut(&event)?;
        event_mut.next_dynamics.push(self.auto_id);
        self.dynamics.insert(self.auto_id, dynmaic);
        self.auto_id += 1;
        Some(self.auto_id - 1)
    }

   
}

#[test]
fn testt() {
    env_logger::init();
    use fns::add_frp_fns;

    let mut vm = EvalRT::new();
    vm.init();
    add_frp_fns(&mut vm);
    let mut system = FRPSystem::default();
    let e0 = system.new_event(None);
    vm.global_context().push_var("eRoot", Variable::Int(e0 as i64));

    let frp_ptr = &mut system as *mut FRPSystem as *mut u8;
    vm.global_context().push_var("*FRPSYSTEM*", Variable::UserData(frp_ptr));

    vm.eval_file("tests/1.clj").unwrap();

    system.fire(e0, &Variable::Nil, &mut vm).unwrap();
    system.fire(e0, &Variable::Nil, &mut vm).unwrap();
    system.fire(e0, &Variable::Nil, &mut vm).unwrap();
    system.fire(e0, &Variable::Nil, &mut vm).unwrap();
}