use crate::tokens::token::Token;

use super::{block_statement::BlockStatement, AstNode, ExpressionNode};

pub struct IfExpression {
    pub token: Token,
    pub condition: ExpressionNode,
    pub concequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl AstNode for IfExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        let else_s = match &self.alternative {
            Some(statement) => format!("else {}", statement.string()),
            None => "".to_string(),
        };
        format!(
            "if {} {} {}",
            self.condition.string(),
            self.concequence.string(),
            else_s
        )
    }
}
