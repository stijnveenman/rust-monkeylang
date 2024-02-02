use std::{
    fmt::{Debug, Display},
    mem,
    rc::Rc,
    sync::Mutex,
};

use crate::{
    ast::{block_statement::BlockStatement, identifier::Identifier, AstNode},
    evaluator::environment::Environment,
};

#[derive(Clone)]
pub struct BuiltinFunction(&'static dyn Fn(Vec<Object>) -> Object);

impl Debug for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuiltinFunction")
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Function(Vec<Identifier>, BlockStatement, Rc<Mutex<Environment>>),
    String(String),
    Builtin(BuiltinFunction),
    Null,
    Return(Box<Object>),
    Error(String),
}

impl Object {
    pub fn is_return(&self) -> bool {
        matches!(self, Object::Return(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
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
            Object::Return(_) => todo!(),
            Object::Error(_) => todo!(),
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::Null => "NULL",
            Object::Function(_, _, _) => "FUNCTION",
            Object::String(_) => "STRING",
            Object::Builtin(_) => "BUILTIN",
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

impl From<&String> for Object {
    fn from(value: &String) -> Self {
        Object::String(value.to_string())
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Object::String(value)
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Null => write!(f, "null"),
            Object::Return(i) => write!(f, "{}", i),
            Object::Function(arguments, block, _) => write!(
                f,
                "fn ({}) {{\n{}\n}}",
                arguments
                    .iter()
                    .map(|a| a.string())
                    .collect::<Vec<_>>()
                    .join(", "),
                block.string()
            ),
            Object::Error(e) => write!(f, "ERROR: {}", e),
            Object::String(s) => write!(f, "{}", s),
            Object::Builtin(_) => write!(f, "Builtin function"),
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::any::Any;

    use crate::{ast::AstNode, evaluator::test::test_eval};

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
            Object::String(v) => {
                assert_eq!(&v, value_any.downcast_ref::<&str>().unwrap())
            }
            Object::Function(_, _, _) => todo!(),
            Object::Null => panic!("called test_object on null object, use test_null if expected"),
            Object::Return(_) => todo!(),
            Object::Error(_) => todo!(),
            Object::Builtin(_) => todo!(),
        }
    }

    pub fn test_null(object: &Object) {
        assert!(matches!(object, Object::Null))
    }

    pub fn test_error(object: &Object, error: &str) {
        let Object::Error(e) = object else {
            panic!("Expected Object::Error, got {:?}", object);
        };
        assert_eq!(e, error)
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2 };";
        let result = test_eval(input);

        let Object::Function(args, body, _) = result else {
            panic!("Expected Object::Function, got {:?}", result);
        };

        assert_eq!(args.len(), 1);

        assert_eq!(args.first().unwrap().string(), "x");

        assert_eq!(body.string(), "(x + 2)");
    }
}
