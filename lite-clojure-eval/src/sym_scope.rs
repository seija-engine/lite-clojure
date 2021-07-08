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

    pub fn pop_scope(&mut self) {
        self.list.pop();
    }

    pub fn top_scope(&mut self) -> &mut SymbolScope {
        &mut self.list[0]
    }

    pub fn top_scope_ref(& self) -> & SymbolScope {
        & self.list[0]
    }

    pub fn last_scope(&mut self) -> &mut SymbolScope {
        self.list.last_mut().unwrap()
    }

    pub fn last_scope_ref(&self) -> &SymbolScope {
        self.list.last().unwrap()
    }

    pub fn deep_find(&self,name:&String) -> Option<Symbol> {
        for scope in self.list.iter().rev() {
            if let Some(sym) = scope.find(name) {
                return Some(sym);
            }
        }
        return  None;
    }
}

#[derive(Default,Debug)]
pub struct SymbolScope {
    lets:Vec<LetScope>,
    syms:HashMap<String,Symbol>
}

impl SymbolScope {
    pub fn push_sym(&mut self,sym:Symbol) {
        if let Some(ls) = self.lets.last_mut() {
            ls.push_sym(sym);
            return;
        }
        self.syms.insert(sym.var_name.clone(), sym);
    }

    pub fn find(&self,name:&String) -> Option<Symbol> {
        for ls in self.lets.iter().rev() {
            if let Some(find) = ls.syms.get(name) {
               return Some(find.clone())
            }
        }
        self.syms.get(name).map(|v | v.clone())
    }

    pub fn push_let(&mut self) {
        self.lets.push(LetScope::default());
    }

    pub fn pop_let(&mut self) {
        self.lets.pop();
    }
}

#[derive(Default,Debug)]
struct LetScope {
  pub  syms:HashMap<String,Symbol>
}

impl LetScope {
    pub fn push_sym(&mut self,sym:Symbol) {
        self.syms.insert(sym.var_name.clone(), sym);
    }
}