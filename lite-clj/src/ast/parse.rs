use std::char;

use super::{errors::ParseError, expr::{Expr,LiteralExpr}, lex_string::LexString};

pub struct ParseAST<'a> {
    source:LexString<'a>
}

impl<'a> ParseAST<'a> {
    pub fn new(code_string:&str) -> ParseAST {
        ParseAST {
            source:LexString::new(code_string)
        }
    }

    pub fn parse(&mut self) -> Result<Expr,ParseError> {
        self.source.skip_whitespace();
        if let Some(chr) = self.next() {
            println!("chr:{:?}",chr);
            let ret = match chr {
                '(' => self.parse_list(),
                chr => {
                    if chr.is_ascii_digit() {
                        return self.parse_number(chr);
                    }

                    
                    Err(ParseError::InvalidChar(chr))
                }
            };
            return  ret;
        };

        return Ok(Expr::Eof);
    }

    pub fn parse_list(&mut self) -> Result<Expr,ParseError> {
        self.source.skip_whitespace();
        let first = self.parse()?;

        todo!()
    }

    pub fn parse_number(&mut self,chr:char) -> Result<Expr,ParseError> {
        dbg!(chr);
        todo!()
    }

    pub fn next(&mut self) -> Option<char> {
        self.source.next()
    }
}


#[test]
fn test_parse() {
    let code_string = "
       3.1415926
    ";
   let mut parser = ParseAST::new(code_string);
   let ret = parser.parse();
}