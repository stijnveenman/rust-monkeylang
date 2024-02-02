#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{evaluator::test::test_eval, object::Object};

    #[rstest]
    #[case("len(\"\")", 1)]
    #[case("len(\"four\")", 4)]
    #[case("len(\"hello world\")", 11)]
    fn test_builtin_len(#[case] input: &str, #[case] result: i64) {
        let evaluated = test_eval(input);

        let Object::Integer(value) = evaluated else {
            panic!("Expected Object::Integer, got {:?}", evaluated);
        };

        assert_eq!(value, result);
    }

    #[rstest]
    #[case("len(1)", "arguments to `len` not supported, got INTEGER")]
    #[case("len(\"one\", \"two\")", "wrong number or arguments. got=1, want=2")]
    fn test_builtin_len_error(#[case] input: &str, #[case] result: &str) {
        let evaluated = test_eval(input);

        let Object::Error(err) = evaluated else {
            panic!("Expected Object::Error, got {:?}", evaluated);
        };

        assert_eq!(err, result);
    }
}
