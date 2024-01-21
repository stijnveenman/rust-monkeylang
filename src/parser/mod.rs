use crate::{
    ast::{
        let_statement::LetStatement, program::Program, ParsableResult, ParseStatement,
        StatementNode,
    },
    tokens::{lexer::Lexer, token::Token},
};

pub struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    fn new(input: String) -> Parser {
        let mut lexer = Lexer::new(input);

        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token: next_token,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let current = self.current_token.clone();
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        current
    }

    pub fn expect_token(&mut self, token: Token) -> Result<Token, String> {
        if self.peek_token.is(&token) {
            Ok(self.next_token())
        } else {
            Err(format!(
                "invalid token, expected '{:?}' got '{:?}'",
                token, self.peek_token,
            ))
        }
    }

    fn parse_statement(&mut self) -> Option<ParsableResult<StatementNode>> {
        match self.current_token {
            Token::LET => Some(LetStatement::parse(self)),
            _ => None,
        }
    }

    fn parse_program(&mut self) -> (Program, Vec<String>) {
        let mut program = Program { statements: vec![] };
        let mut errors = vec![];

        while self.current_token != Token::EOF {
            if let Some(result) = self.parse_statement() {
                match result {
                    Ok(statement) => program.statements.push(statement),
                    Err(e) => errors.push(e),
                }
            }
            self.next_token();
        }

        (program, errors)
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

        let (program, errors) = parser.parse_program();
        let empty: Vec<String> = vec![];

        assert_eq!(errors, empty);
        assert_eq!(program.statements.len(), 3);

        let mut nodes = program.statements.into_iter();
        assert_let(nodes.next().unwrap(), "x");
        assert_let(nodes.next().unwrap(), "y");
        assert_let(nodes.next().unwrap(), "foobar");
    }

    #[test]
    fn test_parser_errors() {
        let input = "
let x 5;
let = 10;
let 838383;
";
        let mut parser = Parser::new(input.into());

        let (_program, errors) = parser.parse_program();

        assert_eq!(errors.len(), 3);
    }
}
