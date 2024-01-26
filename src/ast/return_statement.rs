use crate::{parser::precedence::Precedence, tokens::token::Token};

use super::{AstNode, ExpressionNode, ParseStatement};

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: ExpressionNode,
}

impl AstNode for ReturnStatement {
    fn token(&self) -> &Token {
        &self.token
    }

    fn string(&self) -> String {
        format!("return {};", self.return_value.string())
    }
}

impl ParseStatement for ReturnStatement {
    fn parse(parser: &mut crate::parser::Parser) -> super::ParsableResult<super::StatementNode> {
        let token = parser.current_token.clone();

        let expression = parser.parse_expression(Precedence::LOWEST)?;

        while !parser.current_token.is(&Token::SEMICOLON) {
            parser.next_token();
        }

        Ok(super::StatementNode::ReturnStatement(ReturnStatement {
            token,
            return_value: expression,
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{AstNode, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    fn assert_return(node: StatementNode) {
        let StatementNode::ReturnStatement(statement) = node else {
            panic!("invalid node, expected 'ReturnStatement' got {:?}", node);
        };

        assert_eq!(statement.token(), &Token::RETURN);
    }

    #[test]
    fn test_basic_parser() {
        let input = "
return 5;
return 10;
return 993322;
";
        let mut parser = Parser::new(input.into());

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 3);

        let mut nodes = program.statements.into_iter();
        assert_return(nodes.next().unwrap());
        assert_return(nodes.next().unwrap());
        assert_return(nodes.next().unwrap());
    }
}
