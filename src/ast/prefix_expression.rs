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
    use crate::{
        ast::{integer_literal::test::assert_integer_literal, ExpressionNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    #[test]
    fn test_prefix_expression() {
        let input = "!5;";
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

        assert_eq!(prefix.operator, Token::BANG);
        assert_integer_literal(&prefix.right, 5);
    }
}
