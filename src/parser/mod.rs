use crate::{
    ast::program::Program,
    tokens::{lexer::Lexer, token::Token},
};

struct Parser {
    lexer: Lexer,
    statements: Vec<()>,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(input: String) -> Parser {
        let mut lexer = Lexer::new(input);

        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Parser {
            lexer,
            statements: vec![],
            current_token,
            peek_token: next_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Program {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Expression, Statement, StatementNode},
        parser::Parser,
        tokens::token::Token,
    };

    fn assert_let(node: StatementNode, name: &str) {
        let StatementNode::LetStatement(statement) = node else {
            panic!("invalid node, expected 'let' got {:?}", node);
        };

        assert_eq!(statement.token(), &Token::LET);
        assert_eq!(statement.identifier.value, name);

        let Token::IDENT(literal) = statement.identifier.token() else {
            panic!(
                "expected Token::IDENT in statement.identier, got {:?}",
                statement.identifier.token(),
            );
        };
        assert_eq!(literal, &name);
    }

    #[test]
    fn test_basic_parser() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let mut parser = Parser::new(input.into());

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        let mut nodes = program.statements.into_iter();
        assert_let(nodes.next().unwrap(), "x");
        assert_let(nodes.next().unwrap(), "y");
        assert_let(nodes.next().unwrap(), "foobar");
    }
}
