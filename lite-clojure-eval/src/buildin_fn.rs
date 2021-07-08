use crate::{Variable, eval_rt::{EvalRT}};
 
pub fn print(rt:&EvalRT,args:Vec<Variable>,is_line:bool) -> Variable {
    let mut out_string = String::default();
    let mut idx = 0;
    let args_len = args.len();
    for var in args {
        out_string.push_str(var.show_str(rt).as_str());
        idx += 1;
        if idx != args_len {
            out_string.push(' ');
        }
    }
    if is_line {
        println!("{}",out_string);
    } else {
        print!("{}",out_string);
    }
    Variable::Nil
}

fn is_number_all_int(rt:&EvalRT,args:&Vec<Variable>) -> bool  {
    for arg in args {
        match arg {
          Variable::Float(_) => { return false },
          Variable::Int(_) => (),
          _ => panic!("err number type")
        } 
    }
    return true;
}

fn number_op(rt:&EvalRT,args:&Vec<Variable>,fint:fn(i64,i64) -> i64,ffloat:fn(f64,f64) -> f64) -> Variable {
    let is_int = is_number_all_int(rt, args);
    let mut iter = args.iter();
    if is_int {
        let mut cur:i64 = iter.next().unwrap().cast_int(rt).unwrap();
        while let Some(v) = iter.next() {
            let vnum = v.cast_int(rt).unwrap();
            cur = fint(cur,vnum);
        }
        return Variable::Int(cur);
    } else {
        let mut cur:f64 = iter.next().unwrap().cast_float(rt).unwrap();
        while let Some(v) = iter.next() {
            let vnum = v.cast_float(rt).unwrap();
            cur = ffloat(cur,vnum);
        }
        return Variable::Float(cur);
    }
   
}

pub fn num_add(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        return Variable::Int(0);
    }
    return number_op(rt, &args, |a,b| a + b, |a,b| a + b);
}

pub fn num_sub(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
       panic!("sum number zero args");
    }
    return number_op(rt, &args, |a,b| a - b, |a,b| a - b);
}

pub fn num_mul(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        return Variable::Int(1);
    }
    return number_op(rt, &args, |a,b| a * b, |a,b| a * b);
}

pub fn num_div(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        panic!("num_div number zero args");
    }
    return number_op(rt, &args, |a,b| a / b, |a,b| a / b);
}

pub fn num_lt(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_lt error");
    }
    let a = args[0].cast_float(rt).unwrap();
    let b = args[1].cast_float(rt).unwrap();
    Variable::Bool(a < b)
}

pub fn num_gt(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_gt error");
    }
    let a = args[0].cast_float(rt).unwrap();
    let b = args[1].cast_float(rt).unwrap();
    Variable::Bool(a > b)
}

pub fn num_le(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_le error");
    }
    let a = args[0].cast_float(rt).unwrap();
    let b = args[1].cast_float(rt).unwrap();
    Variable::Bool(a <= b)
}

pub fn num_ge(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_ge error");
    }
    let a = args[0].cast_float(rt).unwrap();
    let b = args[1].cast_float(rt).unwrap();
    Variable::Bool(a >= b)
}

pub fn nth(rt:&EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("nth error");
    }
    let lst = args[0].cast_vec(rt).unwrap();
    let lst_ref:&Vec<Variable> = &lst.borrow();
   
    
    let idx = args[1].cast_int(rt).unwrap() as usize;
    if idx < lst_ref.len() {
        return lst_ref[idx].clone()
    }
    if args.len() > 2 {
       return args[2].clone();
    }
    panic!("index out range");
}