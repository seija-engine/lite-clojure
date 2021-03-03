pub struct Value(ValueRepr);

pub enum ValueRepr {
    Byte(u8),
    Int(i64),
    Float(f64)
}