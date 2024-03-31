use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    LSquirly,
    RSquirly,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LSquirly => write!(f, "LSquirly"),
            Token::RSquirly => write!(f, "RSquirly"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}
