use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    LSquirly,
    RSquirly,
    LSquare,
    RSquare,
    Colon,
    Word(String),
    Comma,
    True,
    False,
    Null,
    Int(String),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LSquirly => write!(f, "LSquirly"),
            Token::RSquirly => write!(f, "RSquirly"),
            Token::LSquare => write!(f, "LSquare"),
            Token::RSquare => write!(f, "RSquare"),
            Token::Colon => write!(f, "Colon"),
            Token::Word(x) => write!(f, "Word({})", x),
            Token::Comma => write!(f, "Comma"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::Null => write!(f, "Null"),
            Token::Int(x) => write!(f, "Int({})", x),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

impl Token {
    pub fn is_open_bracket(&self) -> bool {
        matches!(self, Token::LSquirly | Token::LSquare)
    }

    pub fn is_close_bracket(&self) -> bool {
        matches!(self, Token::RSquirly | Token::RSquare)
    }

    pub fn is_comma(&self) -> bool {
        self == &Token::Comma
    }

    pub fn convert_to_open_bracket(&self) -> &Token {
        match self {
            Token::RSquirly => &Token::LSquirly,
            Token::RSquare => &Token::LSquare,
            x => x,
        }
    }
}
