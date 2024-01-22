use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

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

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{
        ast::{integer_literal::test::assert_integer_literal, ExpressionNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    #[rstest]
    #[case("5 + 5;", 5, Token::PLUS, 5)]
    #[case("5 - 5;", 5, Token::MINUS, 5)]
    #[case("5 * 5;", 5, Token::ASTERISK, 5)]
    #[case("5 / 5;", 5, Token::SLASH, 5)]
    #[case("5 > 5;", 5, Token::GT, 5)]
    #[case("5 < 5;", 5, Token::LT, 5)]
    #[case("5 == 5;", 5, Token::EQ, 5)]
    #[case("5 != 5;", 5, Token::NOT_EQ, 5)]
    // sadly rstest does not work with rust-test
    // https://github.com/rouge8/neotest-rust/pull/57
    fn test_prefix_expression(
        #[case] input: &str,
        #[case] left: u64,
        #[case] token: Token,
        #[case] right: u64,
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
        assert_integer_literal(&infix.left, left);
        assert_integer_literal(&infix.right, right);
    }
}
