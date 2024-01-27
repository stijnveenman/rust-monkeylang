use crate::tokens::token::Token;

use super::{AstNode, ParsePrefix};

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl AstNode for IntegerLiteral {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!("{}", self.value)
    }
}

impl ParsePrefix for IntegerLiteral {
    fn parse_prefix(
        parser: &mut crate::parser::Parser,
    ) -> super::ParsableResult<super::ExpressionNode> {
        let Token::INT(value) = parser.current_token.clone() else {
            return Err(format!("Invalid token {:?}", parser.current_token));
        };

        Ok(super::ExpressionNode::IntegerLiteral(IntegerLiteral {
            token: Token::INT(value),
            value,
        }))
    }
}

#[cfg(test)]
pub mod test {
    use core::panic;

    use crate::{
        ast::{ExpressionNode, StatementNode},
        parser::Parser,
    };

    pub fn assert_integer_literal(expression: &ExpressionNode, value: i64) {
        let ExpressionNode::IntegerLiteral(integer) = expression else {
            panic!(
                "expected IntegerLiteral for expression, got {:?}",
                expression
            );
        };

        assert_eq!(integer.value, value);
    }

    #[test]
    fn test_expression_statement() {
        let input = "5;";
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

        assert_integer_literal(&expression.expression, 5);
    }
}
