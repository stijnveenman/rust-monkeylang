use crate::tokens::token::Token;

use super::{AstNode, ExpressionNode};

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<ExpressionNode>,
    pub operator: Token,
    pub right: Box<ExpressionNode>,
}

impl AstNode for InfixExpression {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        let operator = match self.operator {
            Token::PLUS => "+",
            Token::MINUS => "-",
            Token::ASTERISK => "*",
            Token::SLASH => "/",

            Token::LT => "<",
            Token::GT => ">",

            Token::EQ => "==",
            Token::NOT_EQ => "!=",
            _ => panic!("Invalid operator on token, got {:?}", self.operator),
        };

        format!(
            "({} {} {})",
            self.left.string(),
            operator,
            self.right.string()
        )
    }
}
