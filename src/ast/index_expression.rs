use crate::{
    parser::{precedence::Precedence, Parser},
    tokens::token::Token,
};

use super::{AstNode, ExpressionNode, ParseInfix};

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
}

impl AstNode for IndexExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!("({}[{}])", self.left.string(), self.right.string())
    }
}

impl ParseInfix for IndexExpression {
    fn parse_infix(
        parser: &mut Parser,
        left: ExpressionNode,
    ) -> super::ParsableResult<ExpressionNode> {
        let token = parser.current_token.clone();
        parser.next_token();

        let index = parser.parse_expression(Precedence::LOWEST)?;

        parser.expect_token(Token::RBRACKET)?;

        Ok(ExpressionNode::IndexExpresssion(IndexExpression {
            token,
            left: Box::new(left),
            right: Box::new(index),
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
    fn test_index_expression() {
        let input = "myArray[1 + 1]";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::IndexExpresssion(expression) = &expression.expression else {
            panic!("expected IndexExpresssion for node, got {:?}", node);
        };

        test_expression(&expression.left, &"myArray");
        test_infix_expression(&expression.right, 1, Token::PLUS, 1);
    }
}
