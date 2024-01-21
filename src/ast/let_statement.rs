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
