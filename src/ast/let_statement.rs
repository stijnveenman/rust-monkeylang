use crate::tokens::token::Token;

use super::{Expression, Statement};

pub struct LetStatement {
    token: Token,
    identifier: Box<dyn Expression>,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn token(&self) -> &Token {
        &self.token
    }
}
