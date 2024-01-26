use crate::tokens::token::Token;

use super::{AstNode, StatementNode};

#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<StatementNode>,
}

impl AstNode for BlockStatement {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|i| i.string())
            .collect::<Vec<_>>()
            .join("")
    }
}
