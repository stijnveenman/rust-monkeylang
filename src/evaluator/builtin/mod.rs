use std::fmt::Debug;

use crate::object::Object;

use self::{first::builtin_first, last::builtin_last, len::builtin_len, rest::builtin_rest};

pub mod first;
pub mod last;
pub mod len;
pub mod rest;

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
        "first" => &builtin_first,
        "last" => &builtin_last,
        "rest" => &builtin_rest,
        _ => return None,
    })))
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{
        evaluator::test::test_eval,
        object::{
            test::{test_null, test_object},
            Object,
        },
    };

    #[rstest]
    #[case("len(\"\")", 0)]
    #[case("len(\"four\")", 4)]
    #[case("len(\"hello world\")", 11)]
    #[case("len([1, 2, 3])", 3)]
    #[case("len([])", 0)]
    #[case("first([1, 2, 3])", 1)]
    #[case("last([1, 2, 3])", 3)]
    fn test_builtin(#[case] input: &str, #[case] result: i64) {
        println!("{}", input);
        let evaluated = test_eval(input);
        println!("-> {}", evaluated);

        let Object::Integer(value) = evaluated else {
            panic!("Expected Object::Integer, got {:?}", evaluated);
        };

        assert_eq!(value, result);
    }

    #[rstest]
    #[case("len(1)", "arguments to `len` not supported, got INTEGER")]
    #[case("len(\"one\", \"two\")", "wrong number of arguments. got=2, want=1")]
    #[case("first(1)", "argument to `first` must be ARRAY, got INTEGER")]
    #[case("last(1)", "argument to `last` must be ARRAY, got INTEGER")]
    #[case("push(1, 1)", "argument to `push` must be ARRAY, got INTEGER")]
    fn test_builtin_error(#[case] input: &str, #[case] result: &str) {
        println!("{}", input);
        let evaluated = test_eval(input);
        println!("-> {}", evaluated);

        let Object::Error(err) = evaluated else {
            panic!("Expected Object::Error, got {:?}", evaluated);
        };

        assert_eq!(err, result);
    }

    #[rstest]
    #[case("rest([1,2,3])", vec![2,3])]
    #[case("push([[], 1])", vec![1])]
    fn test_builtin_error_array(#[case] input: &str, #[case] expected: Vec<i64>) {
        println!("{}", input);
        let evaluated = test_eval(input);
        println!("-> {}", evaluated);

        let Object::Array(result) = evaluated else {
            panic!("Expected Object::Array, got {:?}", evaluated);
        };

        assert_eq!(expected.len(), result.len());
        for (exp, obj) in expected.iter().zip(result) {
            test_object(&obj, exp);
        }
    }

    #[rstest]
    #[case("last([])")]
    #[case("first([])")]
    #[case("rest([])")]
    fn test_builtin_null(#[case] input: &str) {
        println!("{}", input);
        let evaluated = test_eval(input);

        test_null(&evaluated);
    }
}
