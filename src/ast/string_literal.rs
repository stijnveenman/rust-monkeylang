use crate::tokens::token::Token;

use super::AstNode;

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl AstNode for StringLiteral {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        ast::{ExpressionNode, StatementNode},
        parser::Parser,
    };

    #[test]
    fn test_string_literal() {
        let input = "\"foobar\"";
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

        let ExpressionNode::StringLiteral(s) = expression.expression else {
            panic!(
                "expected StringLiteral for node, got {:?}",
                expression.expression
            );
        };

        assert_eq!(s.value, "foobar");
    }
}
