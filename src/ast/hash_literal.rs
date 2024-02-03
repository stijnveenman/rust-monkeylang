use crate::{
    parser::{precedence::Precedence, Parser},
    tokens::token::Token,
};

use super::{AstNode, ExpressionNode, ParsableResult, ParsePrefix};

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

impl ParsePrefix for HashLiteral {
    fn parse_prefix(parser: &mut Parser) -> ParsableResult<ExpressionNode> {
        let token = parser.current_token.clone();
        let mut v = vec![];

        while !parser.peek_token.is(&Token::RBRACE) {
            parser.next_token();

            let key = parser.parse_expression(Precedence::LOWEST)?;

            parser.expect_token(Token::COLON)?;
            parser.next_token();

            let value = parser.parse_expression(Precedence::LOWEST)?;

            v.push((key, value));

            if !parser.peek_token.is(&Token::RBRACE) {
                parser.expect_token(Token::COMMA)?;
            }
        }

        parser.expect_token(Token::RBRACE)?;

        Ok(ExpressionNode::HashLiteral(HashLiteral { token, map: v }))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{
            infix_expression::test::test_infix_expression, test::test_expression, ExpressionNode,
            StatementNode,
        },
        parser::Parser,
        tokens::token::Token,
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
    fn test_hash_literal_boolean() {
        let input = "{\"true\": true, \"false\": false}";
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

        assert_eq!(hash.map.len(), 2);

        let mut iter = hash.map.into_iter();

        let current = iter.next().unwrap();
        test_expression(&current.0, &"true");
        test_expression(&current.1, &true);

        let current = iter.next().unwrap();
        test_expression(&current.0, &"false");
        test_expression(&current.1, &false);
    }

    #[test]
    fn test_hash_literal_string() {
        let input = "{\"hello\": \"world\", \"foo\": \"bar\"}";
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

        assert_eq!(hash.map.len(), 2);

        let mut iter = hash.map.into_iter();

        let current = iter.next().unwrap();
        test_expression(&current.0, &"hello");
        test_expression(&current.1, &"world");

        let current = iter.next().unwrap();
        test_expression(&current.0, &"foo");
        test_expression(&current.1, &"bar");
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

    #[test]
    fn test_hash_literal_with_expresions() {
        let input = "{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}";
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
        test_infix_expression(&current.1, 0, Token::PLUS, 1);

        let current = iter.next().unwrap();
        test_expression(&current.0, &"two");
        test_infix_expression(&current.1, 10, Token::MINUS, 8);

        let current = iter.next().unwrap();
        test_expression(&current.0, &"three");
        test_infix_expression(&current.1, 15, Token::SLASH, 5);
    }
}
