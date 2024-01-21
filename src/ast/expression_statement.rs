use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: ExpressionNode,
}

impl AstNode for ExpressionStatement {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        self.expression.string()
    }
}
