use crate::{parser::precedence::Precedence, tokens::token::Token};

use super::{AstNode, ExpressionNode, ParseStatement};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: ExpressionNode,
}

impl AstNode for ReturnStatement {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!("return {};", self.return_value.string())
    }
}

impl ParseStatement for ReturnStatement {
    fn parse(parser: &mut crate::parser::Parser) -> super::ParsableResult<super::StatementNode> {
        let token = parser.current_token.clone();
        parser.next_token();

        let expression = parser.parse_expression(Precedence::LOWEST)?;

        if parser.peek_token.is(&Token::SEMICOLON) {
            parser.next_token();
        }

        Ok(super::StatementNode::ReturnStatement(ReturnStatement {
            token,
            return_value: expression,
        }))
    }
}

#[cfg(test)]
mod test {
    use core::fmt;
    use std::any::Any;

    use rstest::rstest;

    use crate::{
        ast::{test::test_expression, AstNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    fn assert_return<T: fmt::Debug + Any>(node: StatementNode, value: &T) {
        let StatementNode::ReturnStatement(statement) = node else {
            panic!("invalid node, expected 'ReturnStatement' got {:?}", node);
        };

        assert_eq!(statement.token(), &Token::RETURN);
        test_expression(&statement.return_value, value)
    }

    #[rstest]
    #[case("return 5;", 5i64)]
    #[case("return true;", true)]
    #[case("return foobar;", "foobar")]
    fn test_basic_parser<T: std::fmt::Debug + Any>(#[case] input: &str, #[case] value: T) {
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let mut nodes = program.statements.into_iter();
        let node = nodes.next().unwrap();

        assert_return(node, &value)
    }
}
