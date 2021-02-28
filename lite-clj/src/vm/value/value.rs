use crate::vm::value::object::Object;
pub struct Value {
    inner:Object
}

impl From<Object> for Value {
    fn from(v: Object) -> Value {
        Value {inner: v }
    }
}