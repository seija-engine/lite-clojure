use lite_clojure_eval::{EvalRT, ExecScope, Variable, run_native_fn};
use anyhow::{Result,anyhow};
use crate::FRPSystem;
use thiserror::{Error};

#[derive(Error,Debug)]
pub enum FRPError {
    #[error("not found system")]
    NotFoundSystem,
    #[error("type cast error")]
    TypeCastError,
    #[error("event not found")]
    EventNotFound
}

pub fn add_frp_fns(vm:&mut EvalRT) {
    vm.global_context().push_native_fn("$>", event_map);
    vm.global_context().push_native_fn("<$>", event_map);
}

fn get_frp_system<'a>(scope:&'a mut ExecScope) -> Result<&'a mut FRPSystem> {
   scope.find_userdata("*FRPSYSTEM*").ok_or(anyhow!(FRPError::NotFoundSystem))
}


fn event_map(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("$>", s, a, |scope,mut args| {
        let frp_system = get_frp_system(scope)?;
        let event_var = args.remove(0);
        let next_var = args.remove(0);
        let event_id = event_var.cast_int().ok_or(anyhow!(FRPError::TypeCastError) )?;
        let next_event_id = frp_system.new_next_event(event_id as u32, next_var).ok_or(anyhow!(FRPError::EventNotFound) )?;
        Ok(Variable::Int(next_event_id as i64))
    })
}
