use crate::{parser::precedence::Precedence, tokens::token::Token};

use super::{
    identifier::Identifier, AstNode, ExpressionNode, ParsableResult, ParseStatement, StatementNode,
};

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    pub value: ExpressionNode,
}

impl AstNode for LetStatement {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!(
            "let {} = {};",
            self.identifier.string(),
            self.value.string()
        )
    }
}

impl ParseStatement for LetStatement {
    fn parse(parser: &mut crate::parser::Parser) -> ParsableResult<StatementNode> {
        let token = parser.current_token.clone();
        let Token::IDENT(ident) = parser.peek_token.clone() else {
            return Err(format!(
                "invalid token, expected 'Token::IDENT' got '{:?}'",
                parser.peek_token
            ));
        };
        parser.next_token();

        parser.expect_token(Token::ASSIGN)?;

        parser.next_token();

        let expression = parser.parse_expression(Precedence::LOWEST)?;

        if parser.peek_token.is(&Token::SEMICOLON) {
            parser.next_token();
        }

        Ok(StatementNode::LetStatement(LetStatement {
            token,
            identifier: Identifier {
                token: Token::IDENT(ident.clone()),
                value: ident,
            },
            value: expression,
        }))
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use rstest::rstest;

    use crate::{
        ast::{test::test_expression, AstNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    fn assert_let<T: std::fmt::Debug + Any>(node: StatementNode, name: &str, value: &T) {
        let StatementNode::LetStatement(statement) = node else {
            panic!("invalid node, expected 'let' got {:?}", node);
        };

        assert_eq!(statement.token(), &Token::LET);
        assert_eq!(statement.identifier.value, name);

        let Token::IDENT(literal) = statement.identifier.token() else {
            panic!(
                "expected Token::IDENT in statement.identier, got {:?}",
                statement.identifier.token(),
            );
        };
        assert_eq!(literal, &name);

        test_expression(&statement.value, value)
    }

    #[rstest]
    #[case("let x = 5;", "x", 5i64)]
    #[case("let y = true;", "y", true)]
    #[case("let foobar = y;", "foobar", "y")]
    fn test_let_expression<T: std::fmt::Debug + 'static>(
        #[case] input: &str,
        #[case] name: &str,
        #[case] value: T,
    ) {
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let mut nodes = program.statements.into_iter();
        assert_let(nodes.next().unwrap(), name, &value);
    }
}
