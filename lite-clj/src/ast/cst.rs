use std::{char, collections::HashMap, ops::Index};
use regex::Regex;
use crate::ast::value::{CONST_KEY,TAG_KEY};
lazy_static!(
    static ref SymbolPat: Regex = Regex::new("[:]?([\\D&&[^/]].*/)?(/|[\\D&&[^/]][^/]*)").unwrap();
);

use lazy_static::lazy_static;

use super::{errors::CSTError, cexpr::{self, CExpr, Number}, lex_string::LexString, utils, value::{Keyword, Symbol}};

pub struct ParseCST<'a> {
    source:LexString<'a>
}

impl<'a> ParseCST<'a> {
    pub fn new(code_string:&str) -> ParseCST {
        ParseCST {
            source:LexString::new(code_string)
        }
    }

    pub fn parse_exprs(&mut self) -> Result<Vec<CExpr>,CSTError> {
        let mut exprs:Vec<CExpr> = Vec::new();
        loop {
            self.source.skip_whitespace();
            if self.source.lookahead(1).is_none() {
                return Ok(exprs);
            }
            let expr = self.parse()?;
            exprs.push(expr);
        }
    }

    pub fn parse(&mut self) -> Result<CExpr,CSTError> {
        self.skip_whitespace();
        if let Some(chr) = self.next() {
            let ret = match chr {
                '\"' => self.parse_string(),
                ';' => self.parse_comment(),
                '^' => self.parse_meta(),
                '\\' => self.parse_char(),
                '(' => self.parse_list(),
                '[' => self.parse_vector(),
                '{' => self.parse_map(),
                '-' => {
                    let nchr = self.source.lookahead(1);
                    if nchr.is_some() && nchr.unwrap().is_ascii_digit() {
                        self.next();
                        return  self.parse_number(nchr.unwrap(),true);
                    } else {
                        Err(CSTError::InvalidChar('-'))
                    }
                },
                chr => {
                    if chr.is_ascii_digit() {
                        return self.parse_number(chr,false);
                    }
                    let token = self.read_token(chr)?;
                    return self.interpret_token(token);
                }
            };
            return  ret;
        };

        return Err(CSTError::ErrEof);
    }

    fn parse_meta(&mut self) -> Result<CExpr,CSTError> {
        let cexpr = self.parse()?;
        let mut meta_list:HashMap<CExpr,CExpr> = HashMap::new();
        match cexpr {
            CExpr::String(str) => {
                let k = CExpr::Keyword(TAG_KEY.clone());
                let v = CExpr::String(str);
                //meta_list.insert(k, v);
            },
            CExpr::Symbol(sym) => {
                //meta_list.push(CExpr::Keyword(TAG_KEY.clone()));
                //meta_list.push(CExpr::Symbol(sym));
            },
            CExpr::Keyword(k) => {
                //meta_list.push(CExpr::Keyword(k));
                //meta_list.push(CExpr::Boolean(true))
            },
            CExpr::Map(map_lst) => {
                //meta_list = map_lst;
            }
            _ => return Err(CSTError::ErrMetadata)
        }

        let mut with_expr = self.parse()?;
        //with_expr.set_meta(meta);
        
        todo!()
    }

    fn read_token(&mut self,chr:char) -> Result<String,CSTError> {
        let mut tok = String::from(chr);
        let chars = self.source.take_while(|chr| !utils::is_whitespace(chr) && !utils::is_terminating_token(chr));
        if let Some(chrs) = chars {
            tok.push_str(chrs);
        }
        Ok(tok)
    }

    fn interpret_token(&mut self,token:String) -> Result<CExpr,CSTError> {
        match token.as_str() {
            "nil"  => return Ok(CExpr::Nil),
            "true" => return Ok(CExpr::Boolean(true)),
            "false" => return Ok(CExpr::Boolean(false)),
            str => self.match_sym(str)
        }
    }

    fn match_sym(&mut self,sym_str:&str) -> Result<CExpr,CSTError> {
        let mcaps = SymbolPat.captures(sym_str);
        if let Some(caps) = mcaps {
           
            let ns = caps.get(1);
            let ns_str = ns.map(|a| a.as_str()).unwrap_or_default();
            let name_str = caps.get(2).map(|s|s.as_str()).unwrap_or_default();
            if ns.is_some() && ns_str.ends_with(":/") || name_str.ends_with(":") || sym_str.match_indices("::").count() > 1 {
                return Err(CSTError::ErrToken(sym_str.to_string()));
            }
            if sym_str.starts_with("::") {
                unimplemented!();
            }
            let is_keyword = sym_str.starts_with(":");
            if is_keyword {
                let strs:String = sym_str.chars().skip(1).collect();
                let sym = Symbol::intern_name(strs.as_str());
                return Ok(CExpr::Keyword(Keyword::intern(sym) ));
            } else {
                return Ok(CExpr::Symbol(Symbol::intern_name(sym_str)));
            }
        } else {
            return Err(CSTError::ErrToken(sym_str.to_string()));
        }
    }

    fn parse_char(&mut self) -> Result<CExpr,CSTError> {
       let mtoken = self.source.take_while(|chr| !utils::is_terminating_token(chr) && !utils::is_whitespace(chr));
       match mtoken {
           Some(tok) => {
               let len = tok.chars().count();
               if len == 1 {
                   let c = tok.chars().next().unwrap();
                   return Ok(CExpr::Char(c));
               }
               match tok {
                   "newline" => return Ok(CExpr::Char('\n')),
                   "space" => return Ok(CExpr::Char(' ')),
                   "tab" => return Ok(CExpr::Char('\t')),
                   "return" => return Ok(CExpr::Char('\r')),
                   _ if tok.starts_with('u') => {
                       unimplemented!()
                   }
                   s => return Err(CSTError::UnsupportedCharacter(s.to_string()))
               }
           },
           None => Err(CSTError::ErrEof) 
       }
    }

    fn parse_comment(&mut self) -> Result<CExpr,CSTError> {
        let str = self.source.take_while(|c| c != '\r' && c != '\n').unwrap_or_default();
        Ok(CExpr::Comment(String::from(str)))
    }

    pub fn parse_list(&mut self) -> Result<CExpr,CSTError> {
        let expr_list = self.read_list(')')?;
        Ok(CExpr::List(expr_list))
    }
    pub fn parse_vector(&mut self) -> Result<CExpr,CSTError> {
        let expr_list = self.read_list(']')?;
        Ok(CExpr::Vector(expr_list))
    }

    pub fn parse_map(&mut self) -> Result<CExpr,CSTError> {
        let expr_list = self.read_list('}')?;
        Ok(CExpr::Map(expr_list))
    }


    fn read_list(&mut self,end_char:char) -> Result<Vec<CExpr>,CSTError> {
        let mut lsts:Vec<CExpr> = vec![];
        loop {
           self.skip_whitespace();
           let next_char = self.source.lookahead(1);
           match next_char {
               Some(chr) => {
                   if chr == end_char {
                       self.source.next();
                       return Ok(lsts);
                   } else {
                       lsts.push(self.parse()?);
                   }
               },
               None => {
                   return Err(CSTError::ErrEof);
               }
           }
          
        }
    }

    fn parse_string(&mut self) -> Result<CExpr,CSTError> {
        let mut acc:String = String::default();
        loop {
            let normals = self.source.take_while(utils::is_normal_string_char);
            normals.map(|str|{ acc.push_str(str); });
            match self.source.lookahead(1) {
                Some('"') => {
                    self.next(); 
                    return  Ok(CExpr::String(acc))
                },
                Some('\\') => {
                    self.next();
                    match self.source.lookahead(1) {
                        Some('\"') => {
                            self.next();
                            acc.push('"');
                        },
                        Some('t') => {
                            self.next();
                            acc.push('\t');
                        },
                        Some('r') => {
                            self.next();
                            acc.push('\r');
                        },
                        Some('n') => {
                            self.next();
                            acc.push('\n');
                        },
                        Some('\\') => {
                            self.next();
                            acc.push('\\');
                        },
                        Some(chr ) => {
                            return  Err(CSTError::ErrCharInGap(chr));
                        }
                        None => return Err(CSTError::ErrEof),
                    }
                }
                Some(_) => return Err(CSTError::ErrLineFeedInString),
                None => return Err(CSTError::ErrEof)
            }
        }
    }

    pub fn parse_number(&mut self,chr1:char,is_neg:bool) -> Result<CExpr,CSTError> {
        let chr2 = self.source.lookahead(1);
        match (chr1,chr2) {
            ('0',Some('x')) => {
                self.next();
                let  hex = self.source.take_while(|chr| chr.is_ascii_hexdigit()).unwrap_or_default();
                if hex.len() == 0 {
                    return Err(CSTError::ErrExpectedHex);
                }
                let mut raw = String::from("0x");
                raw.push_str(hex);
                let n = utils::digits_to_integer_base(16, &hex);
                if is_neg {
                    raw.insert(0, '-')
                }
                let lit = CExpr::Number(raw,Number::Int(if is_neg { -n } else { n }));
                return Ok(lit);
            },
            _ => {
                let mb_int1 = self.integer1(chr1)?;
                let mb_fraction = self.fraction()?;
                match (mb_int1,mb_fraction) {
                    (Some((mut raw,sint)),None) => {
                        if is_neg {
                            raw.insert(0, '-');
                          }
                        let int = utils::digits_to_integer(sint.as_str());
                        let e = self.exponent()?;
                        match e {
                            Some((mut rawe,exp)) => {
                               let mb_f = utils::sci_to_f64(int, exp);
                               if let Some(f) = mb_f {
                                  rawe.insert_str(0, raw.as_str());
                                  if is_neg {
                                    rawe.insert(0, '-');
                                  }
                                  let lit = CExpr::Number(rawe,Number::Float(if is_neg { -f } else { f }));
                                  return Ok(lit);
                               } else {
                                  return Err(CSTError::ErrNumberOutOfRange);
                               }
                            },
                            None => return Ok(CExpr::Number(raw,Number::Int(if is_neg { -int } else { int })))
                         }
                    },
                    (Some((mut raw, sint)),Some((rawf,frac))) => {
                        if is_neg {
                            raw.insert(0, '-')
                          }
                        let mut sint_c = sint.clone();
                        sint_c.push_str(frac.as_str());
                        let val = sint_c.parse::<f64>().unwrap();
                        let mb_e = self.exponent()?;
                        match mb_e {
                            Some((estr,e)) => {
                               let valf = utils::sci_to_f64_(val, e);
                               if let Some(f) = valf {
                                  raw.push_str(rawf.as_str());
                                  raw.push_str(estr.as_str());
                                  
                                  return Ok(CExpr::Number(raw,Number::Float(if is_neg { -f } else { f })));
                               } else {
                                  return Err(CSTError::ErrNumberOutOfRange);
                               }
                            },
                            None => {
                               raw.push_str(rawf.as_str());
                               return Ok(CExpr::Number(raw,Number::Float(if is_neg { -val } else { val })));
                            }
                         }
                    },
                    _ => {
                        let mc = self.source.lookahead(1);
                        let cstr = mc.map(|c| {let mut r = String::default(); r.push(c); r } );
                        return Err(CSTError::ErrLexeme(cstr));
                    }
                }
            }
        }
    }

    fn integer1(&mut self,chr:char) -> Result<Option<(String,String)>,CSTError> {
        match chr {
            '0' => {
                let mb_ch = self.source.lookahead(1);
                match mb_ch {
                    Some(c) if utils::is_number_char(c) => {
                       return  Err(CSTError::ErrLeadingZero); 
                    },
                     _ => {
                        let zero_str = String::from("0");
                        return Ok(Some((zero_str.clone(),zero_str)));
                     }
                }
            }
            chr if utils::is_digit_char(chr) => {
                let (mut raw,mut numstr) = self.digits()?;
                raw.insert(0, chr);
                numstr.insert(0, chr);
                return Ok(Some((raw,numstr)));
            },
            _ => Ok(None)
        }
    }

    fn exponent(&mut self) -> Result<Option<(String,i64)>,CSTError> {
        let mb_chr = self.source.lookahead(1);
        match mb_chr {
           Some('e') => {
              self.next();
              let mb_chr2 = self.source.lookahead(1);
              let (neg,chr) = match mb_chr2 {
                 Some('-') => { self.next(); (true,"-") },
                 Some('+') => { self.next(); (false,"+") },
                 _ => (false,"")
              };
              let mb_int= self.integer()?;
              match mb_int {
                 Some((mut raw,chs)) => {
                    let int = if neg {
                       -utils::digits_to_integer(chs.as_str())
                    } else {
                        utils::digits_to_integer(chs.as_str())
                    };
                    raw.insert_str(0, chr);
                    raw.insert(0, 'e');
                    return Ok(Some((raw,int)));
                 },
                 None => return Err(CSTError::ErrExpectedExponent)
              }
           },
           _ => return Ok(None)
        }
  
       
     }

    fn integer(&mut self) -> Result<Option<(String,String)>,CSTError> {
        let mb_chr = self.source.lookahead(1);
        match mb_chr {
           Some('0') => {
              self.next();
              let mb_chr2 = self.source.lookahead(1);
              match mb_chr2 {
                 Some(c) if utils::is_number_char(c) => return Err(CSTError::ErrLeadingZero),
                 _ => {
                    let zero_str = String::from("0");
                    return Ok(Some((zero_str.clone(),zero_str)));
                 } 
              }
           },
           Some(ch) if utils::is_digit_char(ch)  => {
               self.digits().map(|d|Some(d))
           },
           _ =>  return Ok(None)
        }
     }

    fn fraction(&mut self) -> Result<Option<(String,String)>,CSTError> {
        let chr1 = self.source.lookahead(1);      
        match chr1 {
           Some('.') => {
              self.next();
              let nums = self.source.take_while(utils::is_number_char);
              match nums {
                 Some(str) => {
                    let mut raw = String::from(str);
                    raw.insert(0, '.');
                    let ret:String = raw.chars().filter(|c| *c != '_').collect();
                    return Ok(Some((raw,ret)));
                 }
                 None => {
                    self.source.put_back('.');
                    return Ok(None)
                 }
              }
           },
           _ => return Ok(None)
        }
     }

    fn digits(&mut self) -> Result<(String,String),CSTError>{
        let arr = self.source.take_while(utils  ::is_number_char);
        let raw = arr.unwrap_or_default();
        let num_str:String = raw.chars().filter(|c| *c != '_').collect();
        return Ok((raw.to_owned(),num_str));
     }

    pub fn next(&mut self) -> Option<char> {
        self.source.next()
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(chr) = self.source.lookahead(1) {
            if chr.is_whitespace() || chr == ',' {
                self.next();
            } else {
                break;
            }
        }
    }
}


#[test]
fn test_parse() {
    let code_string = std::fs::read_to_string("tests/clj/test.clj").unwrap();
   let mut parser = ParseCST::new(&code_string);
   let ret = parser.parse_exprs();
   for e in ret.unwrap() {
       println!("{}",e);
   }

   
}
