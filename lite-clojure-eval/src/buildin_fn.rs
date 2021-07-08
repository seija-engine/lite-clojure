use crate::{Variable, eval_rt::{EvalRT}};
 
pub fn print(rt:&mut EvalRT,args:Vec<Variable>,is_line:bool) -> Variable {
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

fn is_number_all_int(rt:&mut EvalRT,args:&Vec<Variable>) -> bool  {
    for arg in args {
        match arg {
          Variable::Float(_) => { return false },
          Variable::Int(_) => (),
          _ => panic!("err number type")
        } 
    }
    return true;
}

fn number_op(rt:&mut EvalRT,args:&Vec<Variable>,fint:fn(i64,i64) -> i64,ffloat:fn(f64,f64) -> f64) -> Variable {
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

pub fn num_add(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        return Variable::Int(0);
    }
    return number_op(rt, &args, |a,b| a + b, |a,b| a + b);
}

pub fn num_sub(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
       panic!("sum number zero args");
    }
    return number_op(rt, &args, |a,b| a - b, |a,b| a - b);
}

pub fn num_mul(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        return Variable::Int(1);
    }
    return number_op(rt, &args, |a,b| a * b, |a,b| a * b);
}

pub fn num_div(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        panic!("num_div number zero args");
    }
    return number_op(rt, &args, |a,b| a / b, |a,b| a / b);
}

pub fn num_lt(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_lt error");
    }
    let a = args[0].cast_float(rt).unwrap();
    let b = args[1].cast_float(rt).unwrap();
    Variable::Bool(a < b)
}

pub fn num_gt(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_gt error");
    }
    let a = args[0].cast_float(rt).unwrap();
    let b = args[1].cast_float(rt).unwrap();
    Variable::Bool(a > b)
}

pub fn num_le(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_le error");
    }
    let a = args[0].cast_float(rt).unwrap();
    let b = args[1].cast_float(rt).unwrap();
    Variable::Bool(a <= b)
}

pub fn num_ge(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_ge error");
    }
    let a = args[0].cast_float(rt).unwrap();
    let b = args[1].cast_float(rt).unwrap();
    Variable::Bool(a >= b)
}

pub fn nth(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
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

pub fn var_set(rt:&mut EvalRT,mut args: Vec<Variable>) -> Variable {
   if args.len() < 2 {
       panic!("var_set error");
   }
   let var_name = args.remove(0).cast_var(rt).unwrap();
   let set_val = args.remove(0);
   let len = rt.sym_maps.list.len();
   let scope = &rt.sym_maps.list[len - 2];
   
   let find_sym = scope.find(&var_name).or(rt.sym_maps.top_scope().find(&var_name));
 
   if let Some(sym ) = find_sym {
       if let Some(bind_value) = &sym.bind_value {
           *bind_value.borrow_mut() = set_val;
       } else {
           rt.stack[sym.index()] = set_val;
       }
   } else {
       eprintln!("not found var {}",&var_name);
   }
   Variable::Nil
}