use crate::{
    ast::{ExpressionNode, Node, StatementNode},
    object::Object,
    tokens::token::Token,
};

pub fn eval(node: Node) -> Object {
    match node {
        Node::Statement(statement) => eval_statement(statement),
        Node::Expression(expression) => eval_expression(expression),
        Node::Program(program) => eval_statements(&program.statements),
    }
}

fn eval_expression(expression: &ExpressionNode) -> Object {
    match expression {
        ExpressionNode::Identifier(_) => todo!(),
        ExpressionNode::IntegerLiteral(i) => i.value.into(),
        ExpressionNode::BooleanLiteral(i) => i.value.into(),
        ExpressionNode::PrefixExpression(i) => {
            let right = eval(i.right.as_ref().into());

            eval_prefix(&i.operator, right)
        }
        ExpressionNode::InfixExpression(i) => {
            let left = eval(i.left.as_ref().into());
            let right = eval(i.left.as_ref().into());
            eval_infix(&i.operator, left, right)
        }
        ExpressionNode::IfExpression(_) => todo!(),
        ExpressionNode::FunctionExpression(_) => todo!(),
        ExpressionNode::CallExpression(_) => todo!(),
    }
}

fn eval_infix_integer(operator: &Token, left: i64, right: i64) -> Object {
    match operator {
        Token::PLUS => (left + right).into(),
        Token::MINUS => (left - right).into(),
        Token::ASTERISK => (left * right).into(),
        Token::SLASH => (left / right).into(),
        Token::GT => (left > right).into(),
        Token::LT => (left < right).into(),
        Token::EQ => (left == right).into(),
        Token::NOT_EQ => (left != right).into(),
        _ => Object::Null,
    }
}

fn eval_infix(operator: &Token, left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_infix_integer(operator, left, right)
        }
        _ => Object::Null,
    }
}

fn eval_prefix(operator: &Token, right: Object) -> Object {
    match operator {
        Token::BANG => eval_bang(right),
        Token::MINUS => eval_minus(right),
        _ => Object::Null,
    }
}

fn eval_minus(right: Object) -> Object {
    match right {
        Object::Integer(i) => (-i).into(),
        _ => Object::Null,
    }
}

fn eval_bang(right: Object) -> Object {
    match right {
        Object::Boolean(b) => (!b).into(),
        Object::Null => true.into(),
        _ => false.into(),
    }
}

fn eval_statement(statement: &StatementNode) -> Object {
    match statement {
        StatementNode::LetStatement(_) => todo!(),
        StatementNode::ReturnStatement(_) => todo!(),
        StatementNode::ExpressionStatement(expression) => eval_expression(&expression.expression),
    }
}

fn eval_statements(statements: &Vec<StatementNode>) -> Object {
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

        eval((&program).into())
    }

    #[rstest]
    // simple Integer tests
    #[case("5", 5i64)]
    #[case("10", 10i64)]
    #[case("-5", -5i64)]
    #[case("-10", -10i64)]
    // simple Boolean tests
    #[case("true", true)]
    #[case("false", false)]
    // Bang tests
    #[case("!true", false)]
    #[case("!false", true)]
    #[case("!5", false)]
    #[case("!!true", true)]
    #[case("!!false", false)]
    #[case("!!5", true)]
    // infix calculations
    #[case("5 + 5 + 5 + 5 - 10", 10)]
    #[case("2 * 2 * 2 * 2 * 2", 32)]
    #[case("-50 + 100 + -50", 0)]
    #[case("5 * 2 + 10", 20)]
    #[case("5 + 2 * 10", 25)]
    #[case("20 + 2 * -10", 0)]
    #[case("50 / 2 * 2 + 10", 60)]
    #[case("2 * (5 + 10)", 30)]
    #[case("3 * 3 * 3 + 10", 37)]
    #[case("3 * (3 * 3) + 10", 37)]
    #[case("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50)]
    // boolean checks
    #[case("true", true)]
    #[case("false", false)]
    #[case("1 < 2", true)]
    #[case("1 > 2", false)]
    #[case("1 < 1", false)]
    #[case("1 > 1", false)]
    #[case("1 == 1", true)]
    #[case("1 != 1", false)]
    #[case("1 == 2", false)]
    #[case("1 != 2", true)]
    fn test_simple_eval<T: Any>(#[case] input: &str, #[case] value: T) {
        let result = test_eval(input);
        test_object(&result, &value);
    }
}
