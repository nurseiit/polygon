use super::token::Token;
use anyhow::Result;

pub struct Lexer {
    /// input string as bytes vector
    input: Vec<u8>,

    /// current cursor position
    current_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let input_bytes = input.into_bytes();
        Lexer {
            input: input_bytes,
            current_position: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_next_whitespaces();

        let token = match self.get_current_char() {
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            0 => Token::EOF,
            _ => unreachable!(
                "could not match '{}' to any tokens!",
                self.get_current_char()
            ),
        };
        self.move_current_position_once();

        Ok(token)
    }

    fn skip_next_whitespaces(&mut self) {
        while self.get_current_char().is_ascii_whitespace() {
            self.move_current_position_once();
        }
    }

    fn get_current_char(&self) -> u8 {
        if self.current_position < self.input.len() {
            self.input[self.current_position]
        } else {
            0
        }
    }

    fn move_current_position_once(&mut self) {
        self.current_position += 1;
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::Lexer;
    use crate::json_lexer::token::Token;

    fn read_tokens_as_vec(mut lexer: Lexer) -> Vec<Token> {
        let mut actual_tokens = Vec::new();
        while let Ok(token) = lexer.next_token() {
            actual_tokens.push(token);
            if token == Token::EOF {
                break;
            }
        }
        actual_tokens
    }

    #[test]
    fn get_next_token_test() {
        let input = "{}";
        let lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![Token::LSquirly, Token::RSquirly, Token::EOF];
        let actual_tokens = read_tokens_as_vec(lexer);

        println!("expected: {:?}, got: {:?}", expected_tokens, actual_tokens);
        assert_eq!(expected_tokens, actual_tokens);
    }
}
