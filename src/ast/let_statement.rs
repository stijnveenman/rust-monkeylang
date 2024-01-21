use crate::tokens::token::Token;

use super::{identifier::Identifier, ExpressionNode, Statement};

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
