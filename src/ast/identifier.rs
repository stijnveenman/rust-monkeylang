use crate::tokens::token::Token;

use super::Expression;

pub struct Identifier {
    token: Token,
    value: String,
}

impl Expression for Identifier {
    fn token(&self) -> &Token {
        &self.token
    }
}
