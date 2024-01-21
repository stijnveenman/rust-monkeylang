use crate::tokens::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch as char == '_' {
            self.read_char();
        }
        self.input[start..self.position].into()
    }

    fn read_number(&mut self) -> u64 {
        let start = self.position;
        while (self.ch as char).is_numeric() {
            self.read_char();
        }
        self.input[start..self.position]
            .parse()
            .expect("parsing of number failed")
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch as char {
            '=' => Token::ASSIGN,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '+' => Token::PLUS,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '\0' => Token::EOF,
            c if c.is_ascii_alphabetic() => return Token::from_ident(self.read_identifier()),
            c if c.is_numeric() => return Token::INT(self.read_number()),
            _ => Token::ILLEGAL,
        };

        self.read_char();

        token
    }
}

#[test]
fn test_basic_tokens() {
    let input = "=+(){},;";
    let mut lexer = Lexer::new(input.into());

    assert_eq!(lexer.next_token(), Token::ASSIGN);
    assert_eq!(lexer.next_token(), Token::PLUS);
    assert_eq!(lexer.next_token(), Token::LPAREN);
    assert_eq!(lexer.next_token(), Token::RPAREN);
    assert_eq!(lexer.next_token(), Token::LBRACE);
    assert_eq!(lexer.next_token(), Token::RBRACE);
    assert_eq!(lexer.next_token(), Token::COMMA);
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_basic_source() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);";
    let mut lexer = Lexer::new(input.into());

    assert_eq!(lexer.next_token(), Token::LET);
    assert_eq!(lexer.next_token(), Token::IDENT("five".into()));
    assert_eq!(lexer.next_token(), Token::ASSIGN);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::LET);
    assert_eq!(lexer.next_token(), Token::IDENT("ten".into()));
    assert_eq!(lexer.next_token(), Token::ASSIGN);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::LET);
    assert_eq!(lexer.next_token(), Token::IDENT("add".into()));
    assert_eq!(lexer.next_token(), Token::ASSIGN);
    assert_eq!(lexer.next_token(), Token::FUNCTION);
    assert_eq!(lexer.next_token(), Token::LPAREN);
    assert_eq!(lexer.next_token(), Token::IDENT("x".into()));
    assert_eq!(lexer.next_token(), Token::COMMA);
    assert_eq!(lexer.next_token(), Token::IDENT("y".into()));
    assert_eq!(lexer.next_token(), Token::RPAREN);
    assert_eq!(lexer.next_token(), Token::LBRACE);
    assert_eq!(lexer.next_token(), Token::IDENT("x".into()));
    assert_eq!(lexer.next_token(), Token::PLUS);
    assert_eq!(lexer.next_token(), Token::IDENT("y".into()));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::RBRACE);
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::LET);
    assert_eq!(lexer.next_token(), Token::IDENT("result".into()));
    assert_eq!(lexer.next_token(), Token::ASSIGN);
    assert_eq!(lexer.next_token(), Token::IDENT("add".into()));
    assert_eq!(lexer.next_token(), Token::LPAREN);
    assert_eq!(lexer.next_token(), Token::IDENT("five".into()));
    assert_eq!(lexer.next_token(), Token::COMMA);
    assert_eq!(lexer.next_token(), Token::IDENT("ten".into()));
    assert_eq!(lexer.next_token(), Token::RPAREN);
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::EOF);
}
