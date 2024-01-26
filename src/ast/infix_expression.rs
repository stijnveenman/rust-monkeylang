use crate::{
    parser::{precedence::IntoPrecedence, Parser},
    tokens::token::Token,
};

use super::{AstNode, ExpressionNode, ParsableResult, ParseInfix};

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<ExpressionNode>,
    pub operator: Token,
    pub right: Box<ExpressionNode>,
}

impl AstNode for InfixExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        let operator = match self.operator {
            Token::PLUS => "+",
            Token::MINUS => "-",
            Token::ASTERISK => "*",
            Token::SLASH => "/",

            Token::LT => "<",
            Token::GT => ">",

            Token::EQ => "==",
            Token::NOT_EQ => "!=",
            _ => panic!("Invalid operator on token, got {:?}", self.operator),
        };

        format!(
            "({} {} {})",
            self.left.string(),
            operator,
            self.right.string()
        )
    }
}

impl ParseInfix for InfixExpression {
    fn parse_infix(parser: &mut Parser, left: ExpressionNode) -> ParsableResult<ExpressionNode> {
        let token = parser.current_token.clone();
        let operator = parser.current_token.clone();

        parser.next_token();

        let right = parser.parse_expression(operator.precedence())?;

        Ok(ExpressionNode::InfixExpression(InfixExpression {
            token,
            left: left.into(),
            operator,
            right: right.into(),
        }))
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use rstest::rstest;

    use crate::{
        ast::{test::test_expression, AstNode, ExpressionNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    #[rstest]
    #[case("5 + 5;", 5u64, Token::PLUS, 5u64)]
    #[case("5 - 5;", 5u64, Token::MINUS, 5u64)]
    #[case("5 * 5;", 5u64, Token::ASTERISK, 5u64)]
    #[case("5 / 5;", 5u64, Token::SLASH, 5u64)]
    #[case("5 > 5;", 5u64, Token::GT, 5u64)]
    #[case("5 < 5;", 5u64, Token::LT, 5u64)]
    #[case("5 == 5;", 5u64, Token::EQ, 5u64)]
    #[case("5 != 5;", 5u64, Token::NOT_EQ, 5u64)]
    #[case("foobar + barfoo;", "foobar", Token::PLUS, "barfoo")]
    #[case("foobar - barfoo;", "foobar", Token::MINUS, "barfoo")]
    #[case("foobar * barfoo;", "foobar", Token::ASTERISK, "barfoo")]
    #[case("foobar / barfoo;", "foobar", Token::SLASH, "barfoo")]
    #[case("foobar > barfoo;", "foobar", Token::GT, "barfoo")]
    #[case("foobar < barfoo;", "foobar", Token::LT, "barfoo")]
    #[case("foobar == barfoo;", "foobar", Token::EQ, "barfoo")]
    #[case("foobar != barfoo;", "foobar", Token::NOT_EQ, "barfoo")]
    #[case("true == true", true, Token::EQ, true)]
    #[case("true != false", true, Token::NOT_EQ, false)]
    #[case("false == false", false, Token::EQ, false)]
    // sadly rstest does not work with rust-test
    // https://github.com/rouge8/neotest-rust/pull/57
    fn test_infix_expression<T: std::fmt::Debug + Any>(
        #[case] input: &str,
        #[case] left: T,
        #[case] token: Token,
        #[case] right: T,
    ) {
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let mut nodes = program.statements.into_iter();
        let node = nodes.next().unwrap();

        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::InfixExpression(infix) = expression.expression else {
            panic!(
                "expected PrefixExpression for expression, got {:?}",
                expression.expression
            );
        };

        assert_eq!(infix.operator, token);
        test_expression(&infix.left, &left);
        test_expression(&infix.right, &right);
    }

    #[rstest]
    #[case("-a * b", "((-a) * b)")]
    #[case("!-a", "(!(-a))")]
    #[case("a + b + c", "((a + b) + c)")]
    #[case("a + b - c", "((a + b) - c)")]
    #[case("a * b * c", "((a * b) * c)")]
    #[case("a * b / c", "((a * b) / c)")]
    #[case("a + b / c", "(a + (b / c))")]
    #[case("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)")]
    #[case("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)")]
    #[case("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))")]
    #[case("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))")]
    #[case("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))")]
    #[case("true", "true")]
    #[case("false", "false")]
    #[case("3 > 5 == false", "((3 > 5) == false)")]
    #[case("3 < 5 == true", "((3 < 5) == true)")]
    fn test_operator_precedence_parsing(#[case] input: &str, #[case] expected: &str) {
        let mut parser = Parser::new(input.into());
        let (program, errors) = parser.parse_program();

        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);

        assert_eq!(expected, &program.string())
    }
}
