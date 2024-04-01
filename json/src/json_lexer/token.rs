use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    LSquirly,
    RSquirly,
    Colon,
    Word(String),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LSquirly => write!(f, "LSquirly"),
            Token::RSquirly => write!(f, "RSquirly"),
            Token::Colon => write!(f, "Colon"),
            Token::Word(x) => write!(f, "Word({})", x),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

impl Token {
    pub fn is_open_bracket(&self) -> bool {
        match self {
            Token::LSquirly => true,
            _ => false,
        }
    }

    pub fn is_close_bracket(&self) -> bool {
        match self {
            Token::RSquirly => true,
            _ => false,
        }
    }

    pub fn convert_to_open_bracket(&self) -> &Token {
        match self {
            Token::RSquirly => &Token::LSquirly,
            x => x,
        }
    }
}
