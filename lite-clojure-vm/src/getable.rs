use super::Getable;
use super::value::{ValueRepr};

impl<'vm, 'value> Getable<'vm, 'value> for i64 {
    fn from_value(_vm: &'vm super::Thread, value: super::Variants<'value>) -> Self {
        match value.0 {
            ValueRepr::Int(val) => val,
            _ => panic!()
        }
    }
}

impl<'vm, 'value> Getable<'vm, 'value> for u8 {
    fn from_value(_vm: &'vm super::Thread, value: super::Variants<'value>) -> Self {
        match value.0 {
            ValueRepr::Byte(val) => val,
            _ => panic!()
        }
    }
}

impl<'vm, 'value> Getable<'vm, 'value> for f64 {
    fn from_value(_vm: &'vm super::Thread, value: super::Variants<'value>) -> Self {
        match value.0 {
            ValueRepr::Float(val) => val,
            _ => panic!()
        }
    }
}