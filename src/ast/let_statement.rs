use crate::tokens::token::Token;

use super::{identifier::Identifier, ParsableResult, ParseStatement, Statement, StatementNode};

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    //pub value: ExpressionNode,
}

impl Statement for LetStatement {
    fn token(&self) -> &Token {
        &self.token
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

        while !parser.current_token.is(&Token::SEMICOLON) {
            parser.next_token();
        }

        Ok(StatementNode::LetStatement(LetStatement {
            token,
            identifier: Identifier {
                token: Token::IDENT(ident.clone()),
                value: ident,
            },
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Expression, Statement, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    fn assert_let(node: StatementNode, name: &str) {
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
    }

    #[test]
    fn test_basic_parser() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 3);

        let mut nodes = program.statements.into_iter();
        assert_let(nodes.next().unwrap(), "x");
        assert_let(nodes.next().unwrap(), "y");
        assert_let(nodes.next().unwrap(), "foobar");
    }

    #[test]
    fn test_parser_errors() {
        let input = "
let x 5;
let = 10;
let 838383;
";
        let mut parser = Parser::new(input.into());

        let (_program, errors) = parser.parse_program();

        assert_eq!(errors.len(), 3);
    }
}
