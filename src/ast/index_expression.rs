use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
}

impl AstNode for IndexExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!("({}[{}])", self.left.string(), self.right.string())
    }
}
