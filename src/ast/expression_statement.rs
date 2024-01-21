use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: ExpressionNode,
}

impl AstNode for ExpressionStatement {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        self.expression.string()
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use crate::{
        ast::{ExpressionNode, StatementNode},
        parser::Parser,
    };

    #[test]
    fn test_expression_statement() {
        let input = "foobar;";
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

        let ExpressionNode::Identifier(iden) = expression.expression else {
            panic!(
                "expected Identifier for expression, got {:?}",
                expression.expression
            );
        };

        assert_eq!(iden.value, "foobar");
    }
}
