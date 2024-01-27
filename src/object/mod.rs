use std::fmt::Display;

pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl From<i64> for Object {
    fn from(val: i64) -> Self {
        Object::Integer(val)
    }
}

impl From<bool> for Object {
    fn from(val: bool) -> Self {
        Object::Boolean(val)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Null => write!(f, "null"),
        }
    }
}

#[cfg(test)]
pub mod test {
    use core::panic;
    use std::any::Any;

    use super::Object;

    pub fn test_object<T: Any>(object: &Object, val: &T) {
        let value_any = val as &dyn Any;

        match object {
            Object::Integer(i) => {
                assert_eq!(value_any.downcast_ref::<i64>().unwrap(), i)
            }
            Object::Boolean(i) => {
                assert_eq!(value_any.downcast_ref::<bool>().unwrap(), i)
            }
            Object::Null => panic!("called test_object on null object, use test_null if expected"),
        }
    }
}
