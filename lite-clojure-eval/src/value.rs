use std::collections::HashMap;
use serde_json::{Map, Number, Value};
use crate::Variable;


impl Into<Value> for  Variable {
    fn into(self) -> Value {
        match self {
            Variable::Int(i) => Value::Number(i.into()),
            Variable::Float(f) => Value::Number(Number::from_f64(f).unwrap()),
            Variable::Bool(b) => Value::Bool(b),
            Variable::String(ref s) => Value::String(s.borrow().clone()),
            Variable::Keyword(ref s) => Value::String(s.borrow().clone()),
            Variable::Array(ref arr) => {
                Value::Array(arr.borrow().iter().map(|v| v.clone().into()).collect())
            },
            Variable::Map(ref map) => {
                let map_ref:&HashMap<Variable,Variable> = &map.borrow();
                let mut value_map = Map::new();
                for (k,v) in map_ref {
                    let k_str = match k {
                       Variable::String(s) => s.borrow().clone(),
                       Variable::Keyword(s) => s.borrow().clone(),
                       Variable::Int(s) => s.to_string(),
                       Variable::Float(f) => f.to_string(),
                       Variable::Bool(b) => b.to_string(),
                       _ => panic!("err key type"),
                    };
                    let val:Value = v.clone().into();
                    value_map.insert(k_str, val);
                }
                Value::Object(value_map)
            }
            
            _ => Value::Null
        }
    }
}