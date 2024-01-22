use core::panic;

use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug)]
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

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{
        ast::{integer_literal::test::assert_integer_literal, ExpressionNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    #[rstest]
    #[case("!5;", Token::BANG, 5)]
    #[case("-15;", Token::MINUS, 15)]
    fn test_prefix_expression(#[case] input: &str, #[case] token: Token, #[case] value: u64) {
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

        assert_eq!(prefix.operator, token);
        assert_integer_literal(&prefix.right, value);
    }
}