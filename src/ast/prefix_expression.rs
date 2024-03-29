use core::panic;

use crate::{
    parser::{precedence::Precedence, Parser},
    tokens::token::Token,
};

use super::{AstNode, ExpressionNode, ParsableResult, ParsePrefix};

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: Token,
    pub right: Box<ExpressionNode>,
}

impl AstNode for PrefixExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        let operator = match self.operator {
            Token::BANG => '!',
            Token::MINUS => '-',
            _ => panic!("Invalid operator on token, got {:?}", self.operator),
        };
        format!("({}{})", operator, self.right.string())
    }
}

impl ParsePrefix for PrefixExpression {
    fn parse_prefix(parser: &mut Parser) -> ParsableResult<ExpressionNode> {
        let token = parser.current_token.clone();
        let operator = parser.current_token.clone();

        parser.next_token();

        let expression = parser.parse_expression(Precedence::PREFIX)?;

        Ok(ExpressionNode::PrefixExpression(PrefixExpression {
            token,
            operator,
            right: Box::new(expression),
        }))
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use rstest::rstest;

    use crate::{
        ast::{test::test_expression, ExpressionNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    use super::PrefixExpression;

    #[rstest]
    #[case("!5;", Token::BANG, 5i64)]
    #[case("-15;", Token::MINUS, 15i64)]
    #[case("!foobar;", Token::BANG, "foobar")]
    #[case("-foobar;", Token::MINUS, "foobar")]
    #[case("!true;", Token::BANG, true)]
    #[case("!false;", Token::BANG, false)]
    // sadly rstest does not work with rust-test
    // https://github.com/rouge8/neotest-rust/pull/57
    fn test_prefix_expression<T: std::fmt::Debug + Any>(
        #[case] input: &str,
        #[case] token: Token,
        #[case] value: T,
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

        let ExpressionNode::PrefixExpression(prefix) = expression.expression else {
            panic!(
                "expected PrefixExpression for expression, got {:?}",
                expression.expression
            );
        };

        test_prefx(prefix, token, &value)
    }

    fn test_prefx<T: std::fmt::Debug + Any>(expression: PrefixExpression, token: Token, value: &T) {
        assert_eq!(expression.token, token);
        test_expression(&expression.right, value);
    }
}
