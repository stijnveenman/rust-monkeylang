use super::{AstNode, StatementNode};

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<StatementNode>,
}

impl AstNode for Program {
    fn token(&self) -> &crate::tokens::token::Token {
        self.statements
            .first()
            .expect("statements is empty calling token")
            .token()
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|i| i.string())
            .collect::<Vec<_>>()
            .join("")
    }
}
