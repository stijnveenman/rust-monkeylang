use std::fmt::Debug;

use crate::object::Object;

use self::len::builtin_len;

pub mod len;

#[derive(Clone)]
pub struct BuiltinFunction(pub &'static dyn Fn(Vec<Object>) -> Object);

impl Debug for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuiltinFunction")
    }
}

pub fn get_builtin(name: &str) -> Option<Object> {
    Some(Object::Builtin(BuiltinFunction(match name {
        "len" => &builtin_len,
        _ => return None,
    })))
}
