use crate::tokens::token::Token;

use super::{
    identifier::Identifier, ExpressionNode, ParsableResult, ParseStatement, Statement,
    StatementNode,
};

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    pub value: ExpressionNode,
}

impl Statement for LetStatement {
    fn token(&self) -> &Token {
        &self.token
    }
}

impl ParseStatement for LetStatement {
    fn parse(parser: &mut crate::parser::Parser) -> ParsableResult<StatementNode> {
        parser.expect_token(Token::IDENT("".into()))?;
        todo!()
    }
}
