use std::{fmt::Display, mem};

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Error(String),
}

impl Object {
    pub fn is_stop_eval(&self) -> bool {
        matches!(self, Object::Return(_) | Object::Error(_))
    }

    pub fn unwrap(self) -> Object {
        if let Object::Return(value) = self {
            return *value;
        }
        self
    }

    pub fn is(&self, other: &Object) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    pub fn type_str(&self) -> &'static str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::Null => "NULL",
            Object::Return(_) => todo!(),
            Object::Error(_) => todo!(),
        }
    }
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
            Object::Return(i) => write!(f, "{}", i),
            Object::Error(e) => write!(f, "ERROR: {}", e),
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::any::Any;

    use super::Object;

    pub fn test_object<T: Any>(object: &Object, val: &T) {
        let value_any = val as &dyn Any;

        match object {
            Object::Integer(i) => {
                let val = value_any
                    .downcast_ref::<i64>()
                    .copied()
                    .or(value_any.downcast_ref::<i32>().map(|v| i64::from(*v)))
                    .unwrap();
                assert_eq!(&val, i)
            }
            Object::Boolean(i) => {
                assert_eq!(value_any.downcast_ref::<bool>().unwrap(), i)
            }
            Object::Null => panic!("called test_object on null object, use test_null if expected"),
            Object::Return(_) => todo!(),
            Object::Error(_) => todo!(),
        }
    }

    pub fn test_null(object: &Object) {
        assert_eq!(object, &Object::Null)
    }

    pub fn test_error(object: &Object, error: &str) {
        let Object::Error(e) = object else {
            panic!("Expected Object::Error, got {:?}", object);
        };
        assert_eq!(e, error)
    }
}
