use crate::tokens::token::Token;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

impl From<&Token> for Precedence {
    fn from(value: &Token) -> Self {
        match value {
            Token::EQ => Precedence::EQUALS,
            Token::NOT_EQ => Precedence::EQUALS,
            Token::LT => Precedence::LESSGREATER,
            Token::GT => Precedence::LESSGREATER,
            Token::PLUS => Precedence::SUM,
            Token::MINUS => Precedence::SUM,
            Token::SLASH => Precedence::PRODUCT,
            Token::ASTERISK => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}

pub trait IntoPrecedence {
    fn precedence(&self) -> Precedence;
}

impl IntoPrecedence for Token {
    fn precedence(&self) -> Precedence {
        Precedence::from(self)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::precedence::Precedence;

    #[test]
    fn test_precedence() {
        assert!(Precedence::LOWEST < Precedence::EQUALS);
        assert!(Precedence::EQUALS > Precedence::LOWEST);
        assert!(Precedence::EQUALS == Precedence::EQUALS);
    }
}
