
pub fn digits_to_integer_base(base:i64,str:&str) -> i64 {
    let mut n:i64 = 0;
    for c in str.chars() {
      n = n * base + (c.to_digit(16).unwrap() as i64)
    }
    n
 }

 pub fn digits_to_integer(str:&str) -> i64 {
    digits_to_integer_base(10, str)
 }

 pub fn is_number_char(chr:char) -> bool {
    return  (chr >= '0' && chr <= '9') || chr == '_'
 }

 pub fn is_normal_string_char(chr:char) -> bool {
   return  chr != '"' && chr != '\\' && chr != '\r' && chr != '\n'
}

fn is_identifier_char(ch: char) -> bool {
   ch.is_alphanumeric() || "|?<>+-_=^%&$*!.".contains(ch)
}

 pub fn is_digit_char(chr:char) -> bool {
    return  chr >= '0' && chr <= '9';
 }

 pub fn sci_to_f64(c:i64,e:i64) -> Option<f64> {
   if c == 0 {
       return Some(0f64);
   }
   if e > 63 || e < -63 {
       return None;
   }
   if e < 0 {
       Some( c as f64 / 10f64.powi(-e as i32))
   } else {
       Some( (c as f64 * 10f64.powi(e as i32) as f64) as f64)
   }
}

pub fn sci_to_f64_(c:f64,e:i64) -> Option<f64> {
   if c == 0f64 {
       return Some(0f64);
   }
   if e > 63 || e < - 63 {
       return None;
   }
   if e < 0 {
       Some( c  / 10f64.powi(-e as i32))
   } else {
       Some( (c * 10f64.powi(e as i32) as f64) as f64)
   }
}

pub fn is_whitespace(chr:char) -> bool {
    chr.is_whitespace() || chr== ','
}

fn is_spec_char(chr:char) -> bool {
    "\";\'@^`~()[]{}\\%#".contains(chr)
}

pub fn is_terminating_token(ch:char) -> bool {
    return ch != '#' && ch != '\'' && ch != '%' && is_spec_char(ch);
}