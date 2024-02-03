use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug, Clone)]
pub struct HashLiteral {
    pub token: Token,
    pub map: Vec<(ExpressionNode, ExpressionNode)>,
}

impl AstNode for HashLiteral {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        let pairs = self
            .map
            .iter()
            .map(|p| format!("{}:{}", p.0.string(), p.1.string()))
            .collect::<Vec<_>>();

        format!("{{{}}}", pairs.join(", "))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{test::test_expression, ExpressionNode, StatementNode},
        parser::Parser,
    };

    #[test]
    fn test_hash_literal() {
        let input = "{\"one\": 1, \"two\": 2, \"three\": 3}";
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

        let ExpressionNode::HashLiteral(hash) = expression.expression else {
            panic!(
                "expected ExpressionStatement for node, got {:?}",
                expression.expression
            );
        };

        assert_eq!(hash.map.len(), 3);

        let mut iter = hash.map.into_iter();

        let current = iter.next().unwrap();
        test_expression(&current.0, &"one");
        test_expression(&current.1, &1);

        let current = iter.next().unwrap();
        test_expression(&current.0, &"two");
        test_expression(&current.1, &2);

        let current = iter.next().unwrap();
        test_expression(&current.0, &"three");
        test_expression(&current.1, &3);
    }

    #[test]
    fn test_hash_literal_empty() {
        let input = "{}";
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

        let ExpressionNode::HashLiteral(hash) = expression.expression else {
            panic!(
                "expected ExpressionStatement for node, got {:?}",
                expression.expression
            );
        };

        assert_eq!(hash.map.len(), 0);
    }
}
