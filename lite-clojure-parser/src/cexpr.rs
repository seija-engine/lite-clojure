use std::{ fmt};

use super::{meta::{Meta, MetaTable}, value::{Keyword, Symbol}};


#[derive(Debug,Clone)]
pub enum CExpr {
    Nil,
    Boolean(bool),
    Keyword(Keyword),
    String(String),
    Comment(String),
    Number(String,Number),
    Symbol(Symbol),
    Char(char),
    List(Vec<CExpr>),
    Vector(Vec<CExpr>),
    Map(Vec<CExpr>),
    Meta(Vec<CExpr>),
    Quote(Box<CExpr>),
    QuoteVar(Symbol),
    SyntaxQuote(Box<CExpr>),
    Dref(Box<CExpr>),
    UnQuote(Box<CExpr>),
    UnQuoteS(Box<CExpr>)
}


impl CExpr {
    pub fn set_meta(&mut self,meta:Meta<CExpr>,table:&mut MetaTable<CExpr>) {
        match self {
            CExpr::Symbol(sym) => {
                let index = table.add_meta(meta);
                sym.set_meta(index)
            }
            _ => ()
        }
    }

    pub fn is_iseq(&self) -> bool {
        match self {
            CExpr::List(_) =>  true,
            CExpr::Quote(_) => true,
            CExpr::UnQuote(_) => true,
            CExpr::Dref(_) => true,
            _ => false
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            CExpr::String(_) => true,
            _ => false
        }
    }

    pub fn is_vec(&self) -> bool {
        match self {
            CExpr::Vector(_) => true,
            _ => false
        }
    }

    pub fn cast_string(self) -> Result<String,Self> {
        match self {
            CExpr::String(s) => Ok(s),
            _ => Err(self)
        }
    } 

    pub fn cast_symbol(self) -> Result<Symbol,Self> {
        match self {
            CExpr::Symbol(s) => Ok(s),
            _ => Err(self)
        }
    }

    pub fn take_list(self) -> Option<Vec<CExpr>>  {
        match self {
            CExpr::Vector(lst) => Some(lst),
            CExpr::List(vec) => Some(vec),
            CExpr::Quote(b) => (*b).take_list(),
            CExpr::Dref(b) => (*b).take_list(),
            CExpr::UnQuote(b) => (*b).take_list(),
            CExpr::UnQuoteS(b) => (*b).take_list(),
            _ => None
        }
    }

    pub fn seq_first(&self) -> Option<&CExpr>  {
        match self {
            CExpr::List(lst) => lst.first(),
            _ => None
        }
    }

    pub fn cast_sym(&self) -> Option<&Symbol> {
        match self {
            CExpr::Symbol(sym) => Some(sym),
            _ => None
        }
    }
}

impl fmt::Display for CExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CExpr::Nil => write!(f,"nil"),
            CExpr::QuoteVar(s) => write!(f,"#'{}",s),
            CExpr::Boolean(b) => write!(f,"{}",b),
            CExpr::Keyword(kv) => write!(f,"{}",kv),
            CExpr::String(str) => write!(f,"\"{}\"",str),
            CExpr::Comment(comment) => write!(f,";{}\r\n",comment),
            CExpr::Number(raw,_) => write!(f,"{}",raw),
            CExpr::Symbol(sym) => {
                if let Some(m) = sym.meta {
                    write!(f,"sym({},{})",sym,m)
                } else {
                    write!(f,"{}",sym)
                }
            },
            CExpr::Char(chr) => write!(f,"'{}'",chr),
            CExpr::List(lst) => {
                write!(f,"{}",display_vec(lst, '(', ')'))
            },
            CExpr::Vector(lst) => {
                write!(f,"{}",display_vec(lst, '[', ']'))
            },
            CExpr::Map(lst) => {
                write!(f,"{}",display_vec(lst, '{', '}'))
            },
            CExpr::Meta(lst) => {
                write!(f,"meta{}",display_vec(lst, '(', ')'))
            },
            CExpr::Quote(expr) => {
                write!(f,"{}",expr)
            },
            CExpr::Dref(expr) => {
                write!(f,"@{}",expr)
            },
            CExpr::SyntaxQuote(expr) => write!(f,"`{}",expr),
            CExpr::UnQuote(expr) => write!(f,"`~{}",expr),
            CExpr::UnQuoteS(expr) => write!(f,"`~@{}",expr)
        }
    }
}

fn display_vec(lst:&Vec<CExpr>,start_chr:char,end_chr:char) -> String {
    let mut ret_str = String::from(start_chr);
    let mut index = 0;
    for e in lst {
        let e_str = format!("{}",e);
        ret_str.push_str(e_str.as_str());
        if index < lst.len() - 1 {
            ret_str.push(' ');
        }
        index += 1;
    }
    ret_str.push(end_chr);
    ret_str
}

#[derive(Debug,Clone)]
pub enum Number {
    Int(i64),
    Float(f64)
}
