use crate::tokens::token::Token;

use super::Expression;

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn token(&self) -> &Token {
        &self.token
    }
}
