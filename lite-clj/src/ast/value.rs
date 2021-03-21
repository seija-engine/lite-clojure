use std::fmt;
#[derive(Debug)]
pub struct Symbol {
    ns:Option<String>,
    name:String,
}

impl Symbol {
    pub fn intern(ns:Option<String>,name:String) -> Symbol {
        Symbol {
            ns,
            name
        }
    }

    pub fn intern_name(nsname:&str) -> Symbol {
        let i = nsname.find('/');
        if i.is_none() || nsname == "/" {
            return Symbol {
                ns:None,
                name:nsname.to_string()
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
            name
        }
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

#[derive(Debug)]
pub struct Keyword {
    sym:Symbol
}

impl Keyword {
    pub fn intern(sym:Symbol) -> Keyword {
        Keyword {sym }
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