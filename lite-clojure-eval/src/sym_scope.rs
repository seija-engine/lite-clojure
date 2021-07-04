use std::{collections::HashMap, string, sync::Arc};
use crate::variable::Symbol;

pub struct SymbolScopes {
    list:Vec<SymbolScope>
}

impl SymbolScopes {

    pub fn new() -> SymbolScopes {
        SymbolScopes {
            list:vec![SymbolScope::default()]
        }     
    }

    pub fn push_scope(&mut self) {
        self.list.push(SymbolScope::default())
    }

    pub fn top_scope(&mut self) -> &mut SymbolScope {
        &mut self.list[0]
    }

    pub fn last_scope(&mut self) -> &mut SymbolScope {
        self.list.last_mut().unwrap()
    }
}

#[derive(Default,Debug)]
pub struct SymbolScope {
    syms:HashMap<Arc<String>,Symbol>
}

impl SymbolScope {
    pub fn push_sym(&mut self,sym:Symbol) {
        self.syms.insert(sym.var_name.clone(), sym);
    }
}