use std::{collections::HashMap, path::PathBuf};
use crate::{exec_context::ExecContext, buildin_fn, Variable};
#[derive(Default)]
pub struct EvalModules {
    pub search_path:Vec<PathBuf>,
    modules:HashMap<String,FileModule>,
    pub(crate) prelude: ExecContext
}

impl EvalModules {
    
    pub fn init(&mut self) {
        
        self.prelude.push_native_fn("var-set", buildin_fn::var_set);
        self.prelude.push_native_fn("print", |rt,args| buildin_fn::print(rt,args,false));
        self.prelude.push_native_fn("println", |rt,args| buildin_fn::print(rt,args,true));
        self.prelude.push_native_fn("require", buildin_fn::require);
        self.prelude.push_native_fn("+", buildin_fn::num_add);
        self.prelude.push_native_fn("-", buildin_fn::num_sub);
        self.prelude.push_native_fn("*", buildin_fn::num_mul);
        self.prelude.push_native_fn("/", buildin_fn::num_div);

        self.prelude.push_native_fn("<", buildin_fn::num_lt);
        self.prelude.push_native_fn("<=", buildin_fn::num_le);
        self.prelude.push_native_fn(">", buildin_fn::num_gt);
        self.prelude.push_native_fn(">=", buildin_fn::num_ge);

        self.prelude.push_native_fn("nth", buildin_fn::nth);
        self.prelude.push_native_fn("get", buildin_fn::get);
        self.prelude.push_native_fn("=", buildin_fn::eq);
        self.prelude.push_native_fn("nil?", buildin_fn::is_nil);
        self.prelude.push_native_fn("concat", buildin_fn::concat);
        //mut list
        self.prelude.push_native_fn("conj!", buildin_fn::conj_mut);
        //mut map
        self.prelude.push_native_fn("assoc!", buildin_fn::assoc_mut);
        self.prelude.push_native_fn("dissoc!", buildin_fn::dissoc_mut);
    }

    pub fn find_symbol(&self,qual:Option<&str>,name:&str) -> Option<Variable> {
       if let Some(qual) = qual {
           if let Some(f_mod) = self.modules.get(qual) {
              return f_mod.context.find_local_symbol(name);
           } else {
               log::error!("not found module:{}",qual);
               None
           }
       } else {
           self.prelude.find_local_symbol(name)
       }
    }

    pub fn require_mod(&mut self,mod_name:&str) {
        if self.modules.contains_key(mod_name) {
            return;
        }
        let mut mod_path = mod_name.replace('.', "/");
        mod_path.push_str(".clj");
        let mut file_path = None;
        for path in self.search_path.iter() {
            let cur_path =  path.join(&mod_path);
            if cur_path.exists() {
                file_path = Some(cur_path);
                continue;
            }
        }
        if file_path.is_none() {
            log::error!("not found {}",mod_name);
            return;
        }
        match std::fs::read_to_string(file_path.unwrap()) {
            Ok(code_string) => {
               let file_mod = FileModule::create(mod_name,code_string.as_str(), self);
               self.modules.insert(mod_name.to_string(), file_mod);
            },
            Err(err) => {
                log::error!("load module:{} error:{:?}",mod_name,err);
            },
        }
    }

    pub fn require_mod_str(&mut self,mod_name:&str,code_string:&str) {
        if self.modules.contains_key(mod_name) {
            return;
        }
        let file_mod = FileModule::create(mod_name,code_string, self);
        self.modules.insert(mod_name.to_string(), file_mod);
    }
}

pub struct FileModule {
    context:ExecContext
}

impl FileModule {
    pub fn create(mod_name:&str,code_string:&str,modules:&mut EvalModules) -> Self {
        let mut context = ExecContext::new();
        context.eval_string(mod_name.to_string(),code_string, modules);
        FileModule { context }
    }
}