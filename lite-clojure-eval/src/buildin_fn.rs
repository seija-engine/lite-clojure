use std::{collections::HashMap};


use crate::{Variable,  variable::ExecScope, GcRefCell};
 
pub fn print(_:&mut ExecScope,args:Vec<Variable>,is_line:bool) -> Variable {
    let mut out_string = String::default();
    let mut idx = 0;
    let args_len = args.len();
    for var in args {
        out_string.push_str(var.show_str().as_str());
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

fn is_number_all_int(args:&Vec<Variable>) -> bool  {
    for arg in args {
        match arg {
          Variable::Float(_) => { return false },
          Variable::Int(_) => (),
          err_var => {
              log::error!("err number type {:?}",err_var); 
              return  false; 
          }
        } 
    }
    return true;
}

fn number_op(args:&Vec<Variable>,fint:fn(i64,i64) -> i64,ffloat:fn(f64,f64) -> f64) -> Variable {
    let is_int = is_number_all_int( args);
    let mut iter = args.iter();
    if is_int {
        let mut cur:i64 = iter.next().unwrap().cast_int().unwrap_or(0i64);
        while let Some(v) = iter.next() {
            let vnum = v.cast_int().unwrap();
            cur = fint(cur,vnum);
        }
        return Variable::Int(cur);
    } else {
        let mut cur:f64 = iter.next().unwrap().cast_float().unwrap_or(0f64);
        while let Some(v) = iter.next() {
            let vnum = v.cast_float().unwrap();
            cur = ffloat(cur,vnum);
        }
        return Variable::Float(cur);
    }
   
}

pub fn num_add(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        return Variable::Int(0);
    }
    return number_op( &args, |a,b| a + b, |a,b| a + b);
}

pub fn num_sub(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
       panic!("sum number zero args");
    }
    return number_op( &args, |a,b| a - b, |a,b| a - b);
}

pub fn num_mul(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        return Variable::Int(1);
    }
    return number_op( &args, |a,b| a * b, |a,b| a * b);
}

pub fn num_div(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() == 0 {
        panic!("num_div number zero args");
    }
    return number_op( &args, |a,b| a / b, |a,b| a / b);
}

pub fn num_lt(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_lt error");
    }
    let a = args[0].cast_float().unwrap();
    let b = args[1].cast_float().unwrap();
    Variable::Bool(a < b)
}

pub fn num_gt(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_gt error");
    }
    let a = args[0].cast_float().unwrap();
    let b = args[1].cast_float().unwrap();
    Variable::Bool(a > b)
}

pub fn num_le(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_le error");
    }
    let a = args[0].cast_float().unwrap();
    let b = args[1].cast_float().unwrap();
    Variable::Bool(a <= b)
}

pub fn num_ge(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("num_ge error");
    }
    let a = args[0].cast_float().unwrap();
    let b = args[1].cast_float().unwrap();
    Variable::Bool(a >= b)
}

pub fn nth(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        panic!("nth error");
    }
    let lst = args[0].cast_vec().unwrap();
    let lst_ref:&Vec<Variable> = &lst.borrow();
   
    
    let idx = args[1].cast_int().unwrap() as usize;
    if idx < lst_ref.len() {
        return lst_ref[idx].clone()
    }
    if args.len() > 2 {
       return args[2].clone();
    }
    panic!("index out range");
}

pub fn var_set(rt:&mut ExecScope,mut args: Vec<Variable>) -> Variable {
   if args.len() < 2 {
       panic!("var_set error");
   }
   let var_name = args.remove(0).cast_var().unwrap();
   let set_val = args.remove(0);
   let len = rt.context.sym_maps.list.len();
   let scope = &rt.context.sym_maps.list[len - 2];
   
   let find_sym = scope.find(&var_name).or(rt.context.sym_maps.top_scope().find(&var_name));
 
   if let Some(sym ) = find_sym {
       if let Some(bind_value) = &sym.bind_value {
           *bind_value.borrow_mut() = set_val;
       } else {
           rt.context.stack[sym.index()] = set_val;
       }
   } else {
       eprintln!("not found var {}",&var_name);
   }
   Variable::Nil
}

pub fn get(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    let default = if args.len() > 2 { Some(args[2].clone()) } else {None };
    match &args[0] {
        Variable::Array(arr) => {
            let idx = args[1].cast_int().unwrap() as usize;
            let arr_ref:&Vec<Variable> = &arr.borrow();
            if idx as usize >= arr_ref.len() {
                default.unwrap_or(Variable::Nil) 
            } else {
                arr_ref[idx].clone()
            }
        },
        Variable::Map(hash_map) => {
            let key = &args[1];
            let map_ref:&HashMap<Variable,Variable> = &hash_map.borrow();
            match map_ref.get(key) {
                Some(v) => v.clone(),
                None => default.unwrap_or(Variable::Nil)
            } 
        },
        _ => Variable::Nil
    } 
}

pub fn eq(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    Variable::Bool(args[0] == args[1])
}

pub fn require(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
   if args.len()  == 0 {
       log::error!("require error: zero args");
       return Variable::Nil; 
   }
   if let Some(mod_name) = args[0].cast_string() {
       scope.modules.require_mod(mod_name.borrow().as_str());
   } else {
       log::error!("require error: arg is not string");
   }
   Variable::Nil
}


pub fn is_nil(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() == 0 { return Variable::Bool(true); }
    match args[0] {
        Variable::Nil => Variable::Bool(true),
        _ => Variable::Bool(false)
    }
}

pub fn concat(_:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() == 0 { return Variable::Array(GcRefCell::new(vec![])); }
    match args[0] {
        Variable::Map(_) => {
            todo!()
        },
        Variable::Array(_) => {
            let mut new_arr:Vec<Variable> = vec![];
            for arg in args.iter() {
                match arg {
                    Variable::Array(arr) => {
                        for item in arr.borrow().iter() {
                            new_arr.push(item.clone());
                        }
                    },
                    _ => {}
                }
            }
            return Variable::Array(GcRefCell::new(new_arr));
        },
        _ => Variable::Nil
    }
}

pub fn dissoc_mut(_:&mut ExecScope,mut args:Vec<Variable>) -> Variable {
    if args.len() == 0 { 
        log::error!("dissoc! error:zero args");
        return Variable::Nil 
    }
    let map_var = args.remove(0);
    if let Some(map) = map_var.cast_map() {
        for arg in args.iter() {
            map.borrow_mut().remove(arg);
        }
    }
    map_var
}

pub fn assoc_mut(_:&mut ExecScope,mut args:Vec<Variable>) -> Variable {
    if args.len() < 3 || ((args.len() - 1) % 2) != 0 { 
        log::error!("assoc! error:error args count:{}",args.len());
        return Variable::Nil 
    }
    let map_var = args.remove(0);
    if let Some(map) = map_var.cast_map() {
        for (index,key) in args.iter().step_by(2).enumerate() {
            let value = args[index * 2 + 1].clone();
            map.borrow_mut().insert(key.clone(), value);
        }
    }
    map_var
}

pub fn conj_mut(_:&mut ExecScope,mut args:Vec<Variable>) -> Variable {
    if args.len() == 0 { return Variable::Array(GcRefCell::new(vec![])); }
    if args.len() == 1 { return args.remove(0); }
    if let Some(list) = args.remove(0).cast_vec() {
        for arg in args.iter() {
            list.borrow_mut().push(arg.clone());
        }
    } else {
        log::error!("conj! fst type error");
    }
    Variable::Nil
}