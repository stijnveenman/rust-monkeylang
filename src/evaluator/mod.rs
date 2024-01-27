use crate::{
    ast::{ExpressionNode, Node, StatementNode},
    object::Object,
};

pub fn eval(node: Node) -> Object {
    match node {
        Node::Statement(statement) => eval_statement(statement),
        Node::Expression(expression) => eval_expression(expression),
        Node::Program(program) => eval_statements(program.statements),
    }
}

fn eval_expression(expression: ExpressionNode) -> Object {
    match expression {
        ExpressionNode::Identifier(_) => todo!(),
        ExpressionNode::IntegerLiteral(i) => i.value.into(),
        ExpressionNode::BooleanLiteral(i) => i.value.into(),
        ExpressionNode::PrefixExpression(_) => todo!(),
        ExpressionNode::InfixExpression(_) => todo!(),
        ExpressionNode::IfExpression(_) => todo!(),
        ExpressionNode::FunctionExpression(_) => todo!(),
        ExpressionNode::CallExpression(_) => todo!(),
    }
}

fn eval_statement(statement: StatementNode) -> Object {
    match statement {
        StatementNode::LetStatement(_) => todo!(),
        StatementNode::ReturnStatement(_) => todo!(),
        StatementNode::ExpressionStatement(expression) => eval_expression(expression.expression),
    }
}

fn eval_statements(statements: Vec<StatementNode>) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(statement);
    }

    result
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use rstest::rstest;

    use crate::{
        evaluator::eval,
        object::{test::test_object, Object},
        parser::Parser,
    };

    fn test_eval(input: &str) -> Object {
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();

        let empty: Vec<String> = vec![];
        assert_eq!(errors, empty);

        eval(program.into())
    }

    #[rstest]
    #[case("5", 5u64)]
    #[case("10", 10u64)]
    #[case("true", true)]
    #[case("false", false)]
    fn test_simple_eval<T: Any>(#[case] input: &str, #[case] value: T) {
        let result = test_eval(input);
        test_object(&result, &value);
    }
}
