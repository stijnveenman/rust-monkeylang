use crate::tokens::token::Token;

pub mod identifier;
pub mod let_statement;
pub mod program;

pub trait Statement {
    fn token(&self) -> &Token;
}

pub trait Expression {
    fn token(&self) -> &Token;
}
