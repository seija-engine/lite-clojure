use std::{collections::HashMap, ops::Deref};
use super::{cexpr::CExpr, value::{Keyword, Symbol}};
#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum Metakey {
    String(String),
    Keyword(Keyword),
    Symbol(Symbol)
}

impl Metakey {
    pub fn from_c_expr(k:&CExpr) -> Option<Metakey> {
        match k {
            CExpr::String(str) => Some(Metakey::String(str.clone())),
            CExpr::Symbol(sym) => Some(Metakey::Symbol(sym.deref().clone())),
            CExpr::Keyword(k) => Some(Metakey::Keyword(k.deref().clone())),
            _ => None
        }
    }
}

pub type MetaIndex = u32;
#[derive(Debug,Clone)]
pub struct Meta<E> {
    inner_map:HashMap<Metakey,E>
}

impl Meta<CExpr> {
    pub fn new() -> Meta<CExpr> {
        Meta {
            inner_map:HashMap::new()
        }
    }

    pub fn insert(&mut self,key:Metakey,v:CExpr){
       self.inner_map.insert(key, v);
    }


    pub fn insert_c_expr(&mut self,k:&CExpr,v:&CExpr) -> bool  {
        if let Some(meta_key) = Metakey::from_c_expr(k) {
            self.inner_map.insert(meta_key, v.clone());
            return true;
        }
        return false;
    }
}

#[derive(Debug)]
pub struct MetaTable<E> {
    cur_index:MetaIndex,
    metas:HashMap<MetaIndex,Meta<E>>
}

impl<E> MetaTable<E> {
    pub fn new() -> MetaTable<E> {
        MetaTable { metas:HashMap::new(),cur_index:0 }
    }

    pub fn add_meta(&mut self,meta:Meta<E>) -> MetaIndex {
        self.cur_index += 1;
        self.metas.insert(self.cur_index, meta);
        self.cur_index
    }

    pub fn get_mut(&mut self,idx:MetaIndex) -> &mut Meta<E> {
        self.metas.get_mut(&idx).unwrap()
    }

    pub fn get(&self,idx:MetaIndex) -> &Meta<E> {
        self.metas.get(&idx).unwrap()
    }
}