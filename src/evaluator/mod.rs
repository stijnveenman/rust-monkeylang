use std::{collections::HashMap, rc::Rc, sync::Mutex};

use crate::{
    ast::{
        hash_literal::HashLiteral, if_expression::IfExpression, ExpressionNode, Node, StatementNode,
    },
    object::Object,
    tokens::token::Token,
};

use self::{
    builtin::get_builtin,
    environment::{Enclose, Environment},
};

pub mod builtin;
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
                return value;
            }

            if let Some(builtin) = get_builtin(&i.value) {
                return builtin;
            }

            Object::Error(format!("identifier not found: {}", i.value))
        }
        ExpressionNode::IntegerLiteral(i) => i.value.into(),
        ExpressionNode::BooleanLiteral(i) => i.value.into(),
        ExpressionNode::StringLiteral(i) => (&i.value).into(),
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
        ExpressionNode::CallExpression(expression) => {
            let function = eval_expression(env, &expression.function);
            if function.is_error() {
                return function;
            }

            let arguments = eval_expressions(env, &expression.arguments);
            if let Some(Object::Error(e)) = arguments.first() {
                return Object::Error(e.to_string());
            }

            call_function(function, arguments)
        }
        ExpressionNode::ArrayLiteral(array) => {
            let arguments = eval_expressions(env, &array.expressions);
            if let Some(Object::Error(e)) = arguments.first() {
                return Object::Error(e.to_string());
            }

            Object::Array(arguments)
        }
        ExpressionNode::IndexExpresssion(expression) => {
            let left = eval_expression(env, &expression.left);
            if left.is_error() {
                return left;
            }
            let right = eval_expression(env, &expression.right);
            if right.is_error() {
                return right;
            }

            eval_index(left, right)
        }
        ExpressionNode::HashLiteral(expression) => eval_hash_literal(env, expression),
    }
}

fn eval_hash_literal(env: &Rc<Mutex<Environment>>, expression: &HashLiteral) -> Object {
    let mut hm = HashMap::new();

    for (key, value) in expression.map.iter() {
        let key = eval_expression(env, key);
        if key.is_error() {
            return key;
        }

        if !key.hashable() {
            return Object::Error(format!("unusable hash key: {}", key));
        }

        let value = eval_expression(env, value);
        if value.is_error() {
            return value;
        }

        hm.insert(key, value);
    }

    Object::Hash(hm)
}

fn eval_index(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Array(array), Object::Integer(index)) => {
            let Some(index) = usize::try_from(index).ok() else {
                return Object::Null;
            };
            array.into_iter().nth(index).unwrap_or(Object::Null)
        }
        (Object::Hash(hash), right) => {
            if !right.hashable() {
                return Object::Error(format!("unusable as hash key: {}", right.type_str()));
            }

            hash.get(&right).unwrap_or(&Object::Null).to_owned()
        }
        (Object::Array(_), right) => Object::Error(format!(
            "index operator not supported with: {}",
            right.type_str()
        )),
        (left, _) => Object::Error(format!(
            "index operator not supported on: {}",
            left.type_str()
        )),
    }
}

fn call_function(function: Object, args: Vec<Object>) -> Object {
    if let Object::Builtin(builtin) = function {
        return builtin.0(args);
    }

    let Object::Function(identifiers, body, env) = function else {
        return Object::Error(format!("not a function: {}", function.type_str()));
    };

    let env = env.enclose();

    for (identifier, value) in identifiers.iter().zip(args) {
        env.lock().unwrap().set(identifier.value.to_string(), value);
    }

    let result = eval_statements(&env, &body.statements);
    result.unwrap()
}

fn eval_expressions(env: &Rc<Mutex<Environment>>, expressions: &[ExpressionNode]) -> Vec<Object> {
    let mut results = vec![];

    for exp in expressions {
        let result = eval_expression(env, exp);
        if result.is_error() {
            return vec![result];
        }
        results.push(result)
    }

    results
}

fn eval_if_expression(env: &Rc<Mutex<Environment>>, if_expression: &IfExpression) -> Object {
    let condition = eval(env, if_expression.condition.as_ref().into());
    if condition.is_error() {
        return condition;
    }

    if condition.is_truthy() {
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
        (Object::String(left), Token::PLUS, Object::String(right)) => (left + &right).into(),
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
    // function application
    #[case("let identity = fn(x) { x; }; identity(5);", 5)]
    #[case("let identity = fn(x) { return x; }; identity(5);", 5)]
    #[case("let double = fn(x) { x * 2; }; double(5);", 10)]
    #[case("let add = fn(x, y) { x + y; }; add(5, 5);", 10)]
    #[case("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20)]
    #[case("fn(x) { x; }(5)", 5)]
    // string
    #[case("\"Hello world!\"", "Hello world!")]
    #[case("\"Hello\" + \" \" + \"world!\"", "Hello world!")]
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
    #[case("\"Hello\" - \"Hello\"", "unknown operator: STRING MINUS STRING")]
    fn test_errors(#[case] input: &str, #[case] error: &str) {
        println!("{}", input);
        let result = test_eval(input);
        println!(" -> {:?}", result);
        test_error(&result, error);
    }

    #[test]
    fn test_array_object() {
        let input = "[1, 2 * 2, 3 + 3]";
        let result = test_eval(input);

        let Object::Array(array) = result else {
            panic!("Expected Object::Array for result, got {:?}", result);
        };

        test_object(array.first().unwrap(), &1);
        test_object(array.get(1).unwrap(), &4);
        test_object(array.get(2).unwrap(), &6);
    }

    #[rstest]
    #[case("[1, 2, 3][0]", 1)]
    #[case("[1, 2, 3][1]", 2)]
    #[case("[1, 2, 3][2]", 3)]
    #[case("let i = 0; [1][i];", 1)]
    #[case("[1, 2, 3][1 + 1];", 3)]
    #[case("let myArray = [1, 2, 3]; myArray[2];", 3)]
    #[case("let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];", 6)]
    #[case("let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2)]
    #[case("{\"foo\": 5}[\"foo\"]", 5)]
    #[case("let key = \"foo\"; {\"foo\": 5}[key]", 5)]
    #[case("{5: 5}[5]", 5)]
    #[case("{true: 5}[true]", 5)]
    #[case("{false: 5}[false]", 5)]
    fn test_index_expression(#[case] input: &str, #[case] value: i64) {
        let result = test_eval(input);
        test_object(&result, &value);
    }

    #[rstest]
    #[case("[1, 2, 3][3]")]
    #[case("[1, 2, 3][-1]")]
    #[case("{\"foo\": 5}[\"bar\"]")]
    #[case("{}[\"foo\"]")]
    fn test_index_null(#[case] input: &str) {
        let result = test_eval(input);
        test_null(&result);
    }

    #[test]
    fn test_hash_eval() {
        let input = "let two = \"two\";
    {
        \"one\": 10 - 9,
        two: 1 + 1,
        \"thr\" + \"ee\": 6 / 2,
        4: 4,
        true: 5,
        false: 6
    }";
        let result = test_eval(input);

        let Object::Hash(hm) = result else {
            panic!("Expected Object::Hash for result, got {}", result);
        };

        assert_eq!(hm.len(), 6);

        test_object(hm.get(&Object::String("one".into())).unwrap(), &1);
        test_object(hm.get(&Object::String("two".into())).unwrap(), &2);
        test_object(hm.get(&Object::String("three".into())).unwrap(), &3);
        test_object(hm.get(&Object::Integer(4)).unwrap(), &4);
        test_object(hm.get(&Object::Boolean(true)).unwrap(), &5);
        test_object(hm.get(&Object::Boolean(false)).unwrap(), &6);
    }

    #[rstest]
    #[case(
        "{\"name\": \"Monkey\"}[fn(x) { x }];",
        "unusable as hash key: FUNCTION"
    )]
    fn test_hash_error(#[case] input: &str, #[case] err: &str) {
        let result = test_eval(input);

        test_error(&result, err)
    }
}
