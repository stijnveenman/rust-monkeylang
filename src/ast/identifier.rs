use crate::tokens::token::Token;

use super::AstNode;

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl AstNode for Identifier {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}
