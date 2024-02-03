use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug, Clone)]
pub struct HashLiteral {
    pub token: Token,
    pub map: Vec<(ExpressionNode, ExpressionNode)>,
}

impl AstNode for HashLiteral {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        let pairs = self
            .map
            .iter()
            .map(|p| format!("{}:{}", p.0.string(), p.1.string()))
            .collect::<Vec<_>>();

        format!("{{{}}}", pairs.join(", "))
    }
}
