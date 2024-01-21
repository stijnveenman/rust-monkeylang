use crate::tokens::token::Token;

use super::AstNode;

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
}

impl AstNode for ExpressionStatement {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!("")
    }
}
