use std::{rc::Rc, sync::Mutex};

use crate::{
    ast::{if_expression::IfExpression, ExpressionNode, Node, StatementNode},
    object::Object,
    tokens::token::Token,
};

use self::environment::Environment;

pub mod environment;

pub fn eval(env: &Rc<Mutex<Environment>>, node: Node) -> Object {
    match node {
        Node::Statement(statement) => eval_statement(env, statement),
        Node::Expression(expression) => eval_expression(env, expression),
        Node::Program(program) => eval_statements(env, &program.statements).unwrap(),
    }
}

fn eval_statement(env: &Rc<Mutex<Environment>>, statement: &StatementNode) -> Object {
    match statement {
        StatementNode::LetStatement(statement) => {
            let value = eval(env, (&statement.value).into());
            if value.is_error() {
                return value;
            }
            env.lock()
                .unwrap()
                .set(statement.identifier.value.to_string(), value);
            Object::Null
        }
        StatementNode::ReturnStatement(statement) => {
            let value = eval(env, (&statement.return_value).into());
            if value.is_error() {
                return value;
            }

            Object::Return(Box::new(value))
        }
        StatementNode::ExpressionStatement(expression) => {
            eval_expression(env, &expression.expression)
        }
        StatementNode::BlockStatement(block) => eval_statements(env, &block.statements),
    }
}

fn eval_expression(env: &Rc<Mutex<Environment>>, expression: &ExpressionNode) -> Object {
    match expression {
        ExpressionNode::Identifier(i) => {
            if let Some(value) = env.lock().unwrap().get(&i.value) {
                value
            } else {
                Object::Error(format!("identifier not found: {}", i.value))
            }
        }
        ExpressionNode::IntegerLiteral(i) => i.value.into(),
        ExpressionNode::BooleanLiteral(i) => i.value.into(),
        ExpressionNode::PrefixExpression(i) => {
            let right = eval(env, i.right.as_ref().into());
            if right.is_error() {
                return right;
            }

            eval_prefix(&i.operator, right)
        }
        ExpressionNode::InfixExpression(i) => {
            let left = eval(env, i.left.as_ref().into());
            if left.is_error() {
                return left;
            }

            let right = eval(env, i.right.as_ref().into());
            if right.is_error() {
                return right;
            }

            eval_infix(&i.operator, left, right)
        }
        ExpressionNode::IfExpression(expression) => eval_if_expression(env, expression),
        ExpressionNode::FunctionExpression(expression) => Object::Function(
            expression.parameters.clone(),
            expression.body.clone(),
            env.clone(),
        ),
        ExpressionNode::CallExpression(_) => todo!(),
    }
}

fn eval_if_expression(env: &Rc<Mutex<Environment>>, if_expression: &IfExpression) -> Object {
    let condition = eval(env, if_expression.condition.as_ref().into());
    if condition.is_error() {
        return condition;
    }

    if is_truthy(&condition) {
        return eval_statements(env, &if_expression.concequence.statements);
    }

    if let Some(alternative) = &if_expression.alternative {
        return eval_statements(env, &alternative.statements);
    }

    Object::Null
}

fn eval_integer_infix(operator: &Token, left: i64, right: i64) -> Object {
    match operator {
        Token::PLUS => (left + right).into(),
        Token::MINUS => (left - right).into(),
        Token::ASTERISK => (left * right).into(),
        Token::SLASH => (left / right).into(),
        Token::GT => (left > right).into(),
        Token::LT => (left < right).into(),
        Token::EQ => (left == right).into(),
        Token::NOT_EQ => (left != right).into(),
        _ => Object::Error(format!("unknown operator: INTEGER {:?} INTEGER", operator)),
    }
}

fn eval_infix(operator: &Token, left: Object, right: Object) -> Object {
    match (left, operator, right) {
        (Object::Integer(left), _, Object::Integer(right)) => {
            eval_integer_infix(operator, left, right)
        }
        (Object::Boolean(left), Token::EQ, Object::Boolean(right)) => (left == right).into(),
        (Object::Boolean(left), Token::NOT_EQ, Object::Boolean(right)) => (left != right).into(),
        (left, operator, right) if !left.is(&right) => Object::Error(format!(
            "type mismatch: {} {:?} {}",
            left.type_str(),
            operator,
            right.type_str()
        )),
        (left, operator, right) => Object::Error(format!(
            "unknown operator: {} {:?} {}",
            left.type_str(),
            operator,
            right.type_str()
        )),
    }
}

fn eval_prefix(operator: &Token, right: Object) -> Object {
    match operator {
        Token::BANG => eval_bang(right),
        Token::MINUS => eval_minus(right),
        _ => Object::Error(format!("Unknown operator: {:?}", operator)),
    }
}

fn eval_minus(right: Object) -> Object {
    match right {
        Object::Integer(i) => (-i).into(),
        _ => Object::Error(format!("unknown operator: MINUS {}", right.type_str())),
    }
}

fn is_truthy(object: &Object) -> bool {
    match object {
        Object::Integer(_) => true,
        Object::Boolean(b) => *b,
        Object::Function(_, _, _) => true,
        Object::Return(i) => is_truthy(i),
        Object::Null => false,
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

fn eval_statements(env: &Rc<Mutex<Environment>>, statements: &Vec<StatementNode>) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(env, statement);
        if result.is_return() || result.is_error() {
            return result;
        }
    }

    result
}

#[cfg(test)]
pub mod test {
    use std::any::Any;

    use rstest::rstest;

    use crate::{
        evaluator::{environment::Environment, eval},
        object::{
            test::{test_error, test_null, test_object},
            Object,
        },
        parser::Parser,
    };

    pub fn test_eval(input: &str) -> Object {
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();

        let empty: Vec<String> = vec![];
        assert_eq!(errors, empty);

        let env = Environment::new();
        eval(&env, (&program).into())
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
    // test let statements
    #[case("let a = 5; a;", 5)]
    #[case("let a = 5 * 5; a;", 25)]
    #[case("let a = 5; let b = a; b;", 5)]
    #[case("let a = 5; let b = a; let c = a + b + 5; c;", 15)]
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

    #[rstest]
    #[case("5 + true;", "type mismatch: INTEGER PLUS BOOLEAN")]
    #[case("5 + true; 5;", "type mismatch: INTEGER PLUS BOOLEAN")]
    #[case("-true", "unknown operator: MINUS BOOLEAN")]
    #[case("true + false;", "unknown operator: BOOLEAN PLUS BOOLEAN")]
    #[case("5; true + false; 5", "unknown operator: BOOLEAN PLUS BOOLEAN")]
    #[case(
        "if (10 > 1) { true + false; }",
        "unknown operator: BOOLEAN PLUS BOOLEAN"
    )]
    #[case(
        "
if (10 > 1) {
  if (10 > 1) {
    return true + false;
  }

  return 1;
}
",
        "unknown operator: BOOLEAN PLUS BOOLEAN"
    )]
    #[case("foobar;", "identifier not found: foobar")]
    fn test_errors(#[case] input: &str, #[case] error: &str) {
        println!("{}", input);
        let result = test_eval(input);
        println!(" -> {:?}", result);
        test_error(&result, error);
    }
}
