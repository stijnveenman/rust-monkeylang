use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub token: Token,
    pub expressoins: Vec<ExpressionNode>,
}

impl AstNode for ArrayLiteral {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!(
            "[{}]",
            self.expressoins
                .iter()
                .map(|x| x.string())
                .collect::<Vec<_>>()
                .join(", ")
        )
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
    fn test_array_parser() {
        let input = "[1, 2 * 2, 3 + 3]";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();

        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::ArrayLiteral(array) = &expression.expression else {
            panic!("expected ArrayLiteral for Expressoin, got {:?}", node);
        };

        assert_eq!(array.expressoins.len(), 3);
        test_expression(array.expressoins.first().unwrap(), &1);
        test_infix_expression(array.expressoins.get(1).unwrap(), 2, Token::ASTERISK, 2);
        test_infix_expression(array.expressoins.get(2).unwrap(), 3, Token::PLUS, 3);
    }

    #[test]
    fn test_empty_array() {
        let input = "[]";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();

        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::ArrayLiteral(array) = &expression.expression else {
            panic!("expected ArrayLiteral for Expressoin, got {:?}", node);
        };

        assert_eq!(array.expressoins.len(), 0);
    }
}
