use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<ExpressionNode>,
    pub arguments: Vec<ExpressionNode>,
}
impl AstNode for CallExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!(
            "{} ({})",
            self.function.string(),
            self.arguments
                .iter()
                .map(|a| a.string())
                .collect::<Vec<_>>()
                .join(",")
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
    fn test_call_expression() {
        let input = "add(1, 2 * 3, 4 + 5);";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::CallExpression(expression) = &expression.expression else {
            panic!("expected CallExpression for node, got {:?}", node);
        };

        test_expression(&expression.function, &"add");

        test_expression(expression.arguments.first().unwrap(), &1u64);
        test_infix_expression(
            expression.arguments.get(1).unwrap(),
            2u64,
            Token::ASTERISK,
            3u64,
        );
        test_infix_expression(
            expression.arguments.get(2).unwrap(),
            4u64,
            Token::PLUS,
            5u64,
        );
    }
}
