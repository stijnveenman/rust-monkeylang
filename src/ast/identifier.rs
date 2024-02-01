use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode, ParsePrefix};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl AstNode for Identifier {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl ParsePrefix for Identifier {
    fn parse_prefix(
        parser: &mut crate::parser::Parser,
    ) -> super::ParsableResult<super::ExpressionNode> {
        let Token::IDENT(ident) = parser.current_token.clone() else {
            return Err(format!("Invalid token {:?}", parser.current_token));
        };

        Ok(ExpressionNode::Identifier(Identifier {
            token: Token::IDENT(ident.clone()),
            value: ident,
        }))
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
