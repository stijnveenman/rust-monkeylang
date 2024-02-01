use crate::tokens::token::Token;

use super::{AstNode, ParsePrefix};

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl AstNode for BooleanLiteral {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!("{}", self.value)
    }
}

impl ParsePrefix for BooleanLiteral {
    fn parse_prefix(
        parser: &mut crate::parser::Parser,
    ) -> super::ParsableResult<super::ExpressionNode> {
        Ok(super::ExpressionNode::BooleanLiteral(BooleanLiteral {
            token: parser.current_token.clone(),
            value: parser.current_token.is(&Token::TRUE),
        }))
    }
}

#[cfg(test)]
pub mod test {

    use rstest::rstest;

    use crate::{
        ast::{ExpressionNode, StatementNode},
        parser::Parser,
    };

    pub fn assert_boolean_literal(expression: &ExpressionNode, value: bool) {
        let ExpressionNode::BooleanLiteral(boolean) = expression else {
            panic!(
                "expected BooleanLiteral for expression, got {:?}",
                expression
            );
        };

        assert_eq!(boolean.value, value);
    }

    #[rstest]
    #[case("true;", true)]
    fn test_boolean_statement(#[case] input: &str, #[case] value: bool) {
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

        assert_boolean_literal(&expression.expression, value);
    }
}
