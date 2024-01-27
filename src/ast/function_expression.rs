use crate::{parser::Parser, tokens::token::Token};

use super::{
    block_statement::BlockStatement, identifier::Identifier, AstNode, ExpressionNode,
    ParsableResult, ParsePrefix,
};

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

impl FunctionExpression {
    fn parse_parameters(parser: &mut Parser) -> ParsableResult<Vec<Identifier>> {
        let mut idents = vec![];

        if parser.peek_token.is(&Token::RPAREN) {
            parser.next_token();
            return Ok(idents);
        }

        parser.next_token();

        loop {
            let node = Identifier::parse_prefix(parser)?;
            let ExpressionNode::Identifier(ident) = node else {
                return Err(format!(
                    "Identifier::parse_prefix returned invalid type, got {:?}",
                    node
                ));
            };

            idents.push(ident);

            if !parser.peek_token.is(&Token::COMMA) {
                break;
            }

            parser.next_token();
            parser.next_token();
        }

        parser.expect_token(Token::RPAREN)?;

        Ok(idents)
    }
}

impl ParsePrefix for FunctionExpression {
    fn parse_prefix(
        parser: &mut crate::parser::Parser,
    ) -> super::ParsableResult<super::ExpressionNode> {
        let token = parser.current_token.clone();
        parser.expect_token(Token::LPAREN)?;

        let parameters = FunctionExpression::parse_parameters(parser)?;

        parser.expect_token(Token::LBRACE)?;

        let body = parser.parse_block()?;

        Ok(ExpressionNode::FunctionExpression(FunctionExpression {
            token,
            parameters,
            body,
        }))
    }
}

#[cfg(test)]
mod test {

    use crate::{
        ast::{infix_expression::test::test_infix_expression, ExpressionNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    #[test]
    fn test_if_statement() {
        let input = "fn(x, y) { x + y }";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 1);

        let node = program.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        let ExpressionNode::FunctionExpression(fn_expression) = &expression.expression else {
            panic!("expected FunctionExpression for node, got {:?}", node);
        };

        assert_eq!(fn_expression.parameters.len(), 2);

        assert_eq!(&fn_expression.parameters.first().unwrap().value, &"x");
        assert_eq!(&fn_expression.parameters.get(1).unwrap().value, &"y");

        assert_eq!(fn_expression.body.statements.len(), 1);

        let node = fn_expression.body.statements.first().unwrap();
        let StatementNode::ExpressionStatement(expression) = node else {
            panic!("expected ExpressionStatement for node, got {:?}", node);
        };

        test_infix_expression(&expression.expression, "x", Token::PLUS, "y");
    }
}