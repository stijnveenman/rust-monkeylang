use crate::{parser::precedence::Precedence, tokens::token::Token};

use super::{block_statement::BlockStatement, AstNode, ExpressionNode, ParsePrefix};

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<ExpressionNode>,
    pub concequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl AstNode for IfExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        let else_s = match &self.alternative {
            Some(statement) => format!("else {}", statement.string()),
            None => "".to_string(),
        };
        format!(
            "if {} {} {}",
            self.condition.string(),
            self.concequence.string(),
            else_s
        )
    }
}

impl ParsePrefix for IfExpression {
    fn parse_prefix(parser: &mut crate::parser::Parser) -> super::ParsableResult<ExpressionNode> {
        let token = parser.current_token.clone();

        parser.expect_token(Token::LPAREN)?;

        parser.next_token();

        let condition = parser.parse_expression(Precedence::LOWEST)?;

        parser.expect_token(Token::RPAREN)?;

        parser.expect_token(Token::LBRACE)?;

        let concequence = parser.parse_block()?;

        let alternative = match parser.peek_token.is(&Token::ELSE) {
            true => {
                parser.next_token();

                parser.expect_token(Token::LBRACE)?;

                Some(parser.parse_block()?)
            }
            false => None,
        };

        Ok(ExpressionNode::IfExpression(IfExpression {
            token,
            condition: Box::new(condition),
            concequence,
            alternative,
        }))
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use crate::{
        ast::{
            infix_expression::test::test_infix_expression, test::test_expression, ExpressionNode,
            StatementNode,
        },
        parser::Parser,
        tokens::token::Token,
    };

    #[test]
    fn test_if_statement() {
        let input = "if (x < y) { x }";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::IfExpression(if_expression) = &expression.expression else {
            panic!("expected IfExpression for node, got {:?}", node);
        };

        test_infix_expression(&if_expression.condition, "x", Token::LT, "y");

        assert_eq!(if_expression.concequence.statements.len(), 1);
        let node = if_expression.concequence.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        test_expression(&expression.expression, &"x");
    }

    #[test]
    fn test_if_else_statement() {
        let input = "if (x < y) { x } else { y }";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::IfExpression(if_expression) = &expression.expression else {
            panic!("expected IfExpression for node, got {:?}", node);
        };

        test_infix_expression(&if_expression.condition, "x", Token::LT, "y");

        assert_eq!(if_expression.concequence.statements.len(), 1);
        let node = if_expression.concequence.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        test_expression(&expression.expression, &"x");

        let alternative = &if_expression.alternative.as_ref().unwrap();

        assert_eq!(alternative.statements.len(), 1);
        let node = alternative.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        test_expression(&expression.expression, &"y");
    }
}
