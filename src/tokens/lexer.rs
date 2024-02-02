use crate::tokens::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
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

    fn peek_char(&mut self) -> u8 {
        if self.read_position > self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
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

    fn read_number(&mut self) -> i64 {
        let start = self.position;
        while (self.ch as char).is_numeric() {
            self.read_char();
        }
        self.input[start..self.position]
            .parse()
            .expect("parsing of number failed")
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch as char == '"' || self.ch == 0 {
                break;
            }
        }

        self.input[position..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch as char {
            // operators
            '=' => match self.peek_char() as char {
                '=' => {
                    self.read_char();
                    Token::EQ
                }
                _ => Token::ASSIGN,
            },
            '!' => match self.peek_char() as char {
                '=' => {
                    self.read_char();
                    Token::NOT_EQ
                }
                _ => Token::BANG,
            },
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,

            '<' => Token::LT,
            '>' => Token::GT,

            // delmiters
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,

            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '[' => Token::LBRACKET,
            ']' => Token::RBRACKET,

            '\0' => Token::EOF,
            '"' => Token::STRING(self.read_string()),
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

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
\"foobar\"
\"foo bar\"
";
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
    assert_eq!(lexer.next_token(), Token::BANG);
    assert_eq!(lexer.next_token(), Token::MINUS);
    assert_eq!(lexer.next_token(), Token::SLASH);
    assert_eq!(lexer.next_token(), Token::ASTERISK);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::LT);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::GT);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::IF);
    assert_eq!(lexer.next_token(), Token::LPAREN);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::LT);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::RPAREN);
    assert_eq!(lexer.next_token(), Token::LBRACE);
    assert_eq!(lexer.next_token(), Token::RETURN);
    assert_eq!(lexer.next_token(), Token::TRUE);
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::RBRACE);
    assert_eq!(lexer.next_token(), Token::ELSE);
    assert_eq!(lexer.next_token(), Token::LBRACE);
    assert_eq!(lexer.next_token(), Token::RETURN);
    assert_eq!(lexer.next_token(), Token::FALSE);
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::RBRACE);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::EQ);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::NOT_EQ);
    assert_eq!(lexer.next_token(), Token::INT(9));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::STRING("foobar".into()));
    assert_eq!(lexer.next_token(), Token::STRING("foo bar".into()));
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_full_lexer() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
[1, 2];";
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
    assert_eq!(lexer.next_token(), Token::BANG);
    assert_eq!(lexer.next_token(), Token::MINUS);
    assert_eq!(lexer.next_token(), Token::SLASH);
    assert_eq!(lexer.next_token(), Token::ASTERISK);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::LT);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::GT);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::IF);
    assert_eq!(lexer.next_token(), Token::LPAREN);
    assert_eq!(lexer.next_token(), Token::INT(5));
    assert_eq!(lexer.next_token(), Token::LT);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::RPAREN);
    assert_eq!(lexer.next_token(), Token::LBRACE);
    assert_eq!(lexer.next_token(), Token::RETURN);
    assert_eq!(lexer.next_token(), Token::TRUE);
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::RBRACE);
    assert_eq!(lexer.next_token(), Token::ELSE);
    assert_eq!(lexer.next_token(), Token::LBRACE);
    assert_eq!(lexer.next_token(), Token::RETURN);
    assert_eq!(lexer.next_token(), Token::FALSE);
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::RBRACE);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::EQ);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::INT(10));
    assert_eq!(lexer.next_token(), Token::NOT_EQ);
    assert_eq!(lexer.next_token(), Token::INT(9));
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::LBRACKET);
    assert_eq!(lexer.next_token(), Token::INT(1));
    assert_eq!(lexer.next_token(), Token::COMMA);
    assert_eq!(lexer.next_token(), Token::INT(2));
    assert_eq!(lexer.next_token(), Token::RBRACKET);
    assert_eq!(lexer.next_token(), Token::SEMICOLON);
    assert_eq!(lexer.next_token(), Token::EOF);
}
