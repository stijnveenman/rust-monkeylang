use crate::{ast::Node, object::Object};

pub fn eval(node: Node) -> Object {
    todo!()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{evaluator::eval, object::Object, parser::Parser};

    #[rstest]
    #[case("5", 5)]
    #[case("10", 10)]
    fn test_eval_integer(#[case] input: &str, #[case] value: u64) {}

    fn test_eval(input: &str) -> Object {
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();

        let empty: Vec<String> = vec![];
        assert_eq!(errors, empty);

        let node = program.statements.into_iter().next().unwrap();
        eval(node.into())
    }
}
