use std::fmt;

use super::value::{Keyword, Symbol};

#[derive(Debug)]
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
    Map(Vec<CExpr>)
}

impl fmt::Display for CExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CExpr::Nil => write!(f,"nil"),
            CExpr::Boolean(b) => write!(f,"{}",b),
            CExpr::Keyword(kv) => write!(f,"{}",kv),
            CExpr::String(str) => write!(f,"\"{}\"",str),
            CExpr::Comment(comment) => write!(f,";{}\r\n",comment),
            CExpr::Number(raw,_) => write!(f,"{}",raw),
            CExpr::Symbol(sym) => write!(f,"{}",sym),
            CExpr::Char(chr) => write!(f,"'{}'",chr),
            CExpr::List(lst) => {
                write!(f,"{}",display_vec(lst, '(', ')'))
            },
            CExpr::Vector(lst) => {
                write!(f,"{}",display_vec(lst, '[', ']'))
            },
            CExpr::Map(lst) => {
                write!(f,"{}",display_vec(lst, '{', '}'))
            }
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

#[derive(Debug)]
pub enum Number {
    Int(i64),
    Float(f64)
}


pub enum Expr {
    
}