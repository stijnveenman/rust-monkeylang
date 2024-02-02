use crate::object::Object;

pub fn builtin_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            1
        ));
    };

    match args.into_iter().next().unwrap() {
        Object::String(s) => (s.len() as i64).into(),
        Object::Array(a) => (a.len() as i64).into(),
        e => Object::Error(format!(
            "arguments to `len` not supported, got {}",
            e.type_str()
        )),
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{evaluator::test::test_eval, object::Object};

    #[rstest]
    #[case("len(\"\")", 0)]
    #[case("len(\"four\")", 4)]
    #[case("len(\"hello world\")", 11)]
    #[case("len([1, 2, 3])", 3)]
    #[case("len([])", 0)]
    fn test_builtin_len(#[case] input: &str, #[case] result: i64) {
        let evaluated = test_eval(input);

        let Object::Integer(value) = evaluated else {
            panic!("Expected Object::Integer, got {:?}", evaluated);
        };

        assert_eq!(value, result);
    }

    #[rstest]
    #[case("len(1)", "arguments to `len` not supported, got INTEGER")]
    #[case("len(\"one\", \"two\")", "wrong number of arguments. got=2, want=1")]
    fn test_builtin_len_error(#[case] input: &str, #[case] result: &str) {
        let evaluated = test_eval(input);

        let Object::Error(err) = evaluated else {
            panic!("Expected Object::Error, got {:?}", evaluated);
        };

        assert_eq!(err, result);
    }
}
