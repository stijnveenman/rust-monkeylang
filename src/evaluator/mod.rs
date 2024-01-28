use crate::{
    ast::{if_expression::IfExpression, ExpressionNode, Node, StatementNode},
    object::Object,
    tokens::token::Token,
};

pub fn eval(node: Node) -> Object {
    match node {
        Node::Statement(statement) => eval_statement(statement),
        Node::Expression(expression) => eval_expression(expression),
        Node::Program(program) => eval_statements(&program.statements).unwrap(),
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
            let right = eval(i.right.as_ref().into());
            eval_infix(&i.operator, left, right)
        }
        ExpressionNode::IfExpression(expression) => eval_if_expression(expression),
        ExpressionNode::FunctionExpression(_) => todo!(),
        ExpressionNode::CallExpression(_) => todo!(),
    }
}

fn eval_if_expression(if_expression: &IfExpression) -> Object {
    let condition = eval(if_expression.condition.as_ref().into());

    if is_truthy(&condition) {
        return eval_statements(&if_expression.concequence.statements);
    }

    if let Some(alternative) = &if_expression.alternative {
        return eval_statements(&alternative.statements);
    }

    Object::Null
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
        (Object::Boolean(left), Object::Boolean(right)) => match operator {
            Token::EQ => (left == right).into(),
            Token::NOT_EQ => (left != right).into(),
            _ => Object::Null,
        },
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

fn is_truthy(object: &Object) -> bool {
    match object {
        Object::Integer(_) => true,
        Object::Boolean(b) => *b,
        Object::Null => false,
        Object::Return(i) => is_truthy(i),
        Object::Error(_) => panic!("called is_truthy on error object"),
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
        StatementNode::ReturnStatement(statement) => {
            let value = eval((&statement.return_value).into());
            Object::Return(Box::new(value))
        }
        StatementNode::ExpressionStatement(expression) => eval_expression(&expression.expression),
        StatementNode::BlockStatement(block) => eval_statements(&block.statements),
    }
}

fn eval_statements(statements: &Vec<StatementNode>) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(statement);
        if result.is_return() {
            return result;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use rstest::rstest;

    use crate::{
        evaluator::eval,
        object::{
            test::{test_null, test_object},
            Object,
        },
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
    // integer boolean checks
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
    // boolean checks
    #[case("true == true", true)]
    #[case("false == false", true)]
    #[case("true == false", false)]
    #[case("true != false", true)]
    #[case("false != true", true)]
    #[case("(1 < 2) == true", true)]
    #[case("(1 < 2) == false", false)]
    #[case("(1 > 2) == true", false)]
    #[case("(1 > 2) == false", true)]
    // test if return statements
    #[case("if (true) { 10 }", 10)]
    #[case("if (1) { 10 }", 10)]
    #[case("if (1 < 2) { 10 }", 10)]
    #[case("if (1 > 2) { 10 } else { 20 }", 20)]
    #[case("if (1 < 2) { 10 } else { 20 }", 10)]
    // return statements
    #[case("return 10;", 10)]
    #[case("return 10; 9;", 10)]
    #[case("return 2 * 5; 9;", 10)]
    #[case("9; return 2 * 5; 9;", 10)]
    #[case(
        "if (10 > 1) {
  if (10 > 1) {
    return 10;
  }

  return 1;
}",
        10
    )]
    fn test_simple_eval<T: Any>(#[case] input: &str, #[case] value: T) {
        println!("{}", input);
        let result = test_eval(input);
        println!(" -> {:?}", result);
        test_object(&result, &value);
    }

    #[rstest]
    #[case("if (1 > 2) { 10 }")]
    #[case("if (false) { 10 }")]
    fn test_nullable(#[case] input: &str) {
        println!("{}", input);
        let result = test_eval(input);
        println!(" -> {:?}", result);
        test_null(&result);
    }
}
