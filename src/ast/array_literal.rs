use crate::{parser::Parser, tokens::token::Token};

use super::{AstNode, ExpressionNode, ParsableResult, ParsePrefix};

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub token: Token,
    pub expressions: Vec<ExpressionNode>,
}

impl AstNode for ArrayLiteral {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!(
            "[{}]",
            self.expressions
                .iter()
                .map(|x| x.string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl ParsePrefix for ArrayLiteral {
    fn parse_prefix(parser: &mut Parser) -> ParsableResult<ExpressionNode> {
        let token = parser.current_token.clone();
        let expressions = parser.parse_expression_list(Token::RBRACKET)?;

        Ok(ExpressionNode::ArrayLiteral(ArrayLiteral {
            token,
            expressions,
        }))
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

        assert_eq!(array.expressions.len(), 3);
        test_expression(array.expressions.first().unwrap(), &1);
        test_infix_expression(array.expressions.get(1).unwrap(), 2, Token::ASTERISK, 2);
        test_infix_expression(array.expressions.get(2).unwrap(), 3, Token::PLUS, 3);
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

        assert_eq!(array.expressions.len(), 0);
    }
}
