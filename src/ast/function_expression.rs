use crate::tokens::token::Token;

use super::{block_statement::BlockStatement, identifier::Identifier, AstNode};

#[derive(Debug)]
pub struct FunctionExpression {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl AstNode for FunctionExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!(
            "fn ({}) {}",
            self.parameters
                .iter()
                .map(|p| p.string())
                .collect::<Vec<_>>()
                .join(","),
            self.body.string()
        )
    }
}

#[cfg(test)]
mod test {

    use crate::{
        ast::{infix_expression::test::test_infix_expression, ExpressionNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    #[test]
    fn test_if_statement() {
        let input = "fn(x, y) { x + y }";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::FunctionExpression(fn_expression) = &expression.expression else {
            panic!("expected FunctionExpression for node, got {:?}", node);
        };

        assert_eq!(fn_expression.parameters.len(), 2);

        assert_eq!(&fn_expression.parameters.first().unwrap().value, &"x");
        assert_eq!(&fn_expression.parameters.get(1).unwrap().value, &"y");

        assert_eq!(fn_expression.body.statements.len(), 1);

        let node = fn_expression.body.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        test_infix_expression(&expression.expression, "x", Token::PLUS, "y");
    }
}
