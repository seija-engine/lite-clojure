use core::str::{Chars};
use std::collections::{VecDeque};

static BACK_LEN:usize = 3usize;

#[derive(Debug)]
pub struct LexString<'a> {
    source:&'a str,
    chars:Chars<'a>,
   
    cache_list:VecDeque<char>,
    ahead_count:usize,
    cur_index:usize,
    char_count:usize,

    line:u64,
    col:u64
}

impl<'a> LexString<'a> {
    pub fn new(str:&'a str) -> Self {
        LexString {
            source:str,
            chars:str.chars(),
            cache_list:VecDeque::default(),
            ahead_count:0,
            cur_index:0,
            char_count:str.chars().count(),
            line:1,
            col:0,
        }
    }

    pub fn remain_len(&self) -> usize {
        self.char_count - self.cur_index
    }

    pub fn next(&mut self) -> Option<char> {
        let chr = self._next();
        if chr == Some('\n') {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }

        chr
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn col(&self) -> u64 {
        self.col
    }

    fn _next(&mut self) -> Option<char> {
      if self.ahead_count > 0 {
          self.cur_index += 1;
          let sub_count = self.sub_ahead_len();
          let chr = self.cache_list[sub_count as usize];
          self.ahead_count -= 1;
          return  Some(chr);
      } else { 
        let next = self.chars.next();
        if let Some(chr) = next {
            if self.cache_list.len() >= BACK_LEN {
                self.cache_list.pop_front();
            }
            self.cache_list.push_back(chr);
            self.cur_index += 1;
        };
        return next;
      }
    }

    pub fn cur_index(&self) -> usize {
        self.cur_index
    }

//       !   #
// ∀ 1 2 3 4 5 6 7
    pub fn lookahead(&mut self,count:usize) -> Option<char> {
       if self.ahead_count > count {
           let sub_len = self.sub_ahead_len() as usize;
           return Some(self.cache_list[sub_len  + count - 1]);
       } else {
           let add_count = count - self.ahead_count;
           for _ in 0..add_count {
               if let Some(chr) = self.chars.next() {
                   self.ahead_count += 1;
                   self.cache_list.push_back(chr);
               } else {
                   return None;
               }
           }
           let sub_len = self.sub_ahead_len() as usize;
           let idx:i32 = sub_len as i32  + count as i32 - 1;
           if idx >= self.cache_list.len() as i32 || idx < 0 {
               return  None;
           }

           return Some(self.cache_list[idx as usize]);
       }
    }

    pub fn lookback(&mut self,count:usize) -> Option<char> {
        let sub_ahead:i32 = self.sub_ahead_len();
        if sub_ahead < 0 {
            return None;
        }
        if sub_ahead - count as i32 >= 0 {
           return Some(self.cache_list[sub_ahead as usize - count - 1]);
        }
        None
    }

    fn sub_ahead_len(&self) -> i32 {
        self.cache_list.len() as i32 - self.ahead_count as i32
    }

    pub fn take_while<F>(&mut self,mut f:F) -> Option<&str> where F:FnMut(char) -> bool {
        let starti = self.cur_index;
        while let Some(chr) = self.lookahead(1) {
            if f(chr) {
                self.next();
            } else {
                if starti == self.cur_index {
                    return  None;
                }
               
                return  Some(self.slice(starti,self.cur_index));
            }
        }
        if self.cur_index != starti {
            return  Some(self.slice(starti,self.cur_index));
        }
        None   
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(chr) = self.lookahead(1) {
            if chr.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    pub fn put_rollback(&mut self,str:&str) {
        for chr in str.chars() {
            self.cache_list.push_back(chr);
            self.ahead_count += 1;
        }
    }

    pub fn put_back(&mut self,chr:char) {
        self.cache_list.push_back(chr);
        self.ahead_count += 1;
    }

    pub fn slice(&self,s:usize,e:usize) -> &str {
        let mut idx = 0usize;
       let mut u8idx_e = 0usize;
       let mut u8idx_s = 0usize;
       for chr in self.source.chars() {
           if idx < s {
               u8idx_s += chr.len_utf8();
           }
           if idx < e {
            u8idx_e += chr.len_utf8();
           } else {
                break;
           }
           idx += 1
       }
       return &self.source[u8idx_s..u8idx_e];
    }
}


#[test]
fn test_string() {
   let mut lex = LexString::new("  1234∀56");
   let aaaa = "∀5";
  
   let aa = lex.take_while(|chr| chr.is_whitespace());
   dbg!(aa.map(|c| c.len()));
   dbg!(lex.next());
}