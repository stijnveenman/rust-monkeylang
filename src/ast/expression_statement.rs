use crate::{parser::precedence::Precedence, tokens::token::Token};

use super::{AstNode, ExpressionNode, ParseStatement};

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

impl ParseStatement for ExpressionStatement {
    fn parse(parser: &mut crate::parser::Parser) -> super::ParsableResult<super::StatementNode> {
        let token = parser.current_token.clone();

        let expression = parser.parse_expression(Precedence::LOWEST)?;

        if parser.peek_token.is(&Token::SEMICOLON) {
            parser.next_token();
        }

        Ok(super::StatementNode::ExpressionStatement(
            ExpressionStatement { token, expression },
        ))
    }
}
