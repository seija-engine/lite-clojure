use lite_clojure_eval::{EvalRT, ExecScope, Variable};

pub fn add_frp_fns(vm:&mut EvalRT) {
    vm.global_context().push_native_fn("$>", event_map_right);
}

fn event_map_right(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    
    Variable::Nil
}
