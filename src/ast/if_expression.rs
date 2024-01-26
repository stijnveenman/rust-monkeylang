use crate::tokens::token::Token;

use super::{block_statement::BlockStatement, AstNode, ExpressionNode};

#[derive(Debug)]
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
}
