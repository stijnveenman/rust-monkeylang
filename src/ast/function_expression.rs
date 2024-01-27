use crate::tokens::token::Token;

use super::{block_statement::BlockStatement, identifier::Identifier, AstNode};

#[derive(Debug)]
pub struct FunctionExpression {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl AstNode for FunctionExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!(
            "fn ({}) {}",
            self.parameters
                .iter()
                .map(|p| p.string())
                .collect::<Vec<_>>()
                .join(","),
            self.body.string()
        )
    }
}
