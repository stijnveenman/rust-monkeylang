use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<ExpressionNode>,
    pub arguments: Vec<ExpressionNode>,
}
impl AstNode for CallExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!(
            "{} ({})",
            self.function.string(),
            self.arguments
                .iter()
                .map(|a| a.string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
