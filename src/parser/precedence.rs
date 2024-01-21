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
