use lite_clojure_eval::{EvalRT, ExecScope, Variable, run_native_fn};
use anyhow::{Result,anyhow};
use crate::FRPSystem;
use crate::errors::{FRPError};


pub fn add_frp_fns(vm:&mut EvalRT) {
    vm.global_context().push_native_fn("$>", event_map);
    vm.global_context().push_native_fn("<$>", event_map);
    vm.global_context().push_native_fn("holdDyn", hold_dynamic);
    vm.global_context().push_native_fn("foldDyn", fold_dynamic);
    vm.global_context().push_native_fn("updated", updated_dynamic);
}

fn get_frp_system<'a>(scope:&'a mut ExecScope) -> Result<&'a mut FRPSystem> {
   scope.find_userdata("*FRPSYSTEM*").ok_or(anyhow!(FRPError::NotFoundSystem))
}


fn event_map(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("<$>", s, a, |scope,mut args| {
        let frp_system = get_frp_system(scope)?;
        let event_var = args.remove(0);
        let next_var = args.remove(0);
        let event_id = event_var.cast_int().ok_or(anyhow!(FRPError::TypeCastError) )?;
        let next_event_id = frp_system.new_next_event(event_id as u32, next_var)
                                          .ok_or(anyhow!(FRPError::EventNotFound) )?;
        Ok(Variable::Int(next_event_id as i64))
    })
}

fn hold_dynamic(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("holdDyn",s,a,|scope,mut args| {
        let frp_system = get_frp_system(scope)?;
        let default_value = args.remove(0);
        let event_id = args.remove(0).cast_int().ok_or(anyhow!(FRPError::TypeCastError))?;
        let dynamic_id = frp_system.create_dynamic(default_value, event_id as u32,None)
                                       .ok_or(anyhow!(FRPError::EventNotFound))?;
        Ok(Variable::Int(dynamic_id as i64))
    })
}

fn fold_dynamic(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("foldDyn", s, a, |scope,mut args| {
        let frp_system = get_frp_system(scope)?;
        let default_value = args.remove(0);
        let event_id = args.remove(0).cast_int().ok_or(anyhow!(FRPError::TypeCastError))?;
        let fn_var = args.remove(0);
        let dynamic_id = frp_system.create_dynamic(default_value, event_id as u32, Some(fn_var))
                                        .ok_or(anyhow!(FRPError::EventNotFound))?;
        Ok(Variable::Int(dynamic_id as i64)) 
    })
}

fn updated_dynamic(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("updated", s, a, |scope,mut args| {
        let frp_system = get_frp_system(scope)?;
        let dynamic_id = args.remove(0).cast_int().ok_or(anyhow!(FRPError::TypeCastError))? as u32;
        let dynamic_updated = frp_system.dynamics.get(&dynamic_id)
                                                            .ok_or(anyhow!(FRPError::DynamicNotFound))?.updated;
        if let Some(updated_id) = dynamic_updated {
            return Ok(Variable::Int(updated_id as i64));
        } else {
            let updated_id = frp_system.new_event(None);
            let mut dynamic = frp_system.dynamics.get_mut(&dynamic_id).ok_or(anyhow!(FRPError::DynamicNotFound))?;
            dynamic.updated = Some(updated_id);
            return Ok(Variable::Int(updated_id as i64))
        }
    })
}