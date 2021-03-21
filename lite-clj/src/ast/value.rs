use std::{collections::HashMap, fmt};
use std::hash::{Hash,Hasher};
use lazy_static::lazy_static;
use std::rc::Rc;

use super::cexpr::CExpr;
lazy_static!(
   pub static ref TAG_KEY: Keyword = Keyword::intern_str(None, "tag");
   pub static ref CONST_KEY:Keyword = Keyword::intern_str(None, "const");
);




#[derive(Debug,Clone)]
pub struct Symbol {
    ns:Option<String>,
    name:String,
    meta:Option<HashMap<CExpr,CExpr>>
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ns.hash(state);
        self.name.hash(state);
    }
}




impl Symbol {
    pub fn intern(ns:Option<String>,name:String) -> Symbol {
        Symbol {
            ns,
            name,
            meta:None
        }
    }

    pub fn intern_name(nsname:&str) -> Symbol {
        let i = nsname.find('/');
        if i.is_none() || nsname == "/" {
            return Symbol {
                ns:None,
                name:nsname.to_string(),
                meta:None
            };
        }
        let mut ns = String::default();
        let mut name = String::default();
        let idx = i.unwrap();
        let mut index = 0;
        for chr in nsname.chars() {
            if index < idx {
                ns.push(chr);
            } else if index > idx {
                name.push(chr)
            }
            index += 1;
        }
        Symbol {
            ns:Some(ns),
            name,
            meta:None
        }
    }

    pub fn set_meta(&mut self,meta:HashMap<CExpr,CExpr>) {
        self.meta = Some(meta)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ns) = &self.ns {
            write!(f,"{}/{}",ns,self.name)
        } else {
            write!(f,"{}",self.name)
        }
    }
}

#[derive(Debug,Clone)]
pub struct Keyword {
    sym:Symbol
}

impl Keyword {
    pub fn intern(sym:Symbol) -> Keyword {
        Keyword {sym }
    }

    pub fn intern_str(ns:Option<&str>,name:&str) -> Keyword {
        let sym = Symbol::intern(ns.map(|s|s.to_string()), name.to_string());
        Self::intern(sym)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,":{}",self.sym)
    }
}

#[test]
fn test_sym() {
    let sym = Symbol::intern_name("aaa/bc/cc");
    dbg!(sym);
}