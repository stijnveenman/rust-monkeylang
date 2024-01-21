use crate::tokens::token::Token;

use super::Statement;

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
}

impl Statement for ExpressionStatement {
    fn token(&self) -> &Token {
        &self.token
    }
}
