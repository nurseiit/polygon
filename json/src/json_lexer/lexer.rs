use super::token::Token;
use anyhow::{bail, Result};

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

    pub fn read_all_tokens(&mut self) -> Result<Vec<Token>> {
        let mut actual_tokens = Vec::new();
        while let Ok(token) = self.next_token() {
            actual_tokens.push(token);
            if actual_tokens.last() == Some(&Token::EOF) {
                break;
            }
        }
        Ok(actual_tokens)
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_next_whitespaces();

        let token = match self.get_current_char() {
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b':' => Token::Colon,
            b'"' => {
                let word = self.read_inside_double_quotes();
                Token::Word(word)
            }
            b',' => Token::Comma,
            b'a'..=b'z' => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "true" => Token::True,
                    "false" => Token::False,
                    "null" => Token::Null,
                    _ => bail!("could not match literal '{}' to any tokens!", ident),
                }
            }
            b'0'..=b'9' => {
                let int = self.read_int();
                Token::Int(int)
            }
            0 => Token::EOF,
            _ => bail!(
                "could not match '{}' to any tokens!",
                self.get_current_char()
            ),
        };
        self.move_current_position_once();

        Ok(token)
    }

    fn read_int(&mut self) -> String {
        let mut chars = vec![];
        let mut current = self.get_current_char();

        while current.is_ascii_digit() {
            chars.push(current);
            if !self.peek_next_char().is_ascii_digit() {
                break;
            }
            self.move_current_position_once();
            current = self.get_current_char();
        }

        String::from_utf8_lossy(&chars).to_string()
    }

    fn read_ident(&mut self) -> String {
        let mut chars = vec![];
        let mut current = self.get_current_char();

        while current.is_ascii_lowercase() {
            chars.push(current);
            if !self.peek_next_char().is_ascii_lowercase() {
                break;
            }
            self.move_current_position_once();
            current = self.get_current_char();
        }

        String::from_utf8_lossy(&chars).to_string()
    }

    fn read_inside_double_quotes(&mut self) -> String {
        let mut chars = vec![];
        self.move_current_position_once();
        let mut current = self.get_current_char();

        while current != b'"' && current != 0 {
            chars.push(current);
            self.move_current_position_once();
            current = self.get_current_char();
        }

        String::from_utf8_lossy(&chars).to_string()
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

    fn peek_next_char(&self) -> u8 {
        if self.current_position + 1 < self.input.len() {
            self.input[self.current_position + 1]
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

    #[test]
    fn get_next_token_minimal_test() {
        let input = "{}";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![Token::LSquirly, Token::RSquirly, Token::EOF];
        let actual_tokens = lexer.read_all_tokens().unwrap();

        println!("expected: {:?}, got: {:?}", expected_tokens, actual_tokens);
        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    fn get_next_token_with_colon() {
        let input = "{:}";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![Token::LSquirly, Token::Colon, Token::RSquirly, Token::EOF];
        let actual_tokens = lexer.read_all_tokens().unwrap();

        println!("expected: {:?}, got: {:?}", expected_tokens, actual_tokens);
        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    fn get_next_token_with_words_and_comma() {
        let input = r#"{ "key":"value", }"#;
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::LSquirly,
            Token::Word(String::from("key")),
            Token::Colon,
            Token::Word(String::from("value")),
            Token::Comma,
            Token::RSquirly,
            Token::EOF,
        ];
        let actual_tokens = lexer.read_all_tokens().unwrap();

        println!("expected: {:?}, got: {:?}", expected_tokens, actual_tokens);
        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    fn string_bool_and_null() {
        let input = r#"{
            "key1": true,
            "key2": false,
            "key3": null,
            "key4": "value"
        }"#;
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::LSquirly,
            Token::Word(String::from("key1")),
            Token::Colon,
            Token::True,
            Token::Comma,
            Token::Word(String::from("key2")),
            Token::Colon,
            Token::False,
            Token::Comma,
            Token::Word(String::from("key3")),
            Token::Colon,
            Token::Null,
            Token::Comma,
            Token::Word(String::from("key4")),
            Token::Colon,
            Token::Word(String::from("value")),
            Token::RSquirly,
            Token::EOF,
        ];
        let actual_tokens = lexer.read_all_tokens().unwrap();

        println!("expected: {:?}, got: {:?}", expected_tokens, actual_tokens);
        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    fn every_primitive() {
        let input = r#"{
            "key1": true,
            "key2": false,
            "key3": null,
            "key4": "value",
            "key5": 667
        }"#;
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::LSquirly,
            Token::Word(String::from("key1")),
            Token::Colon,
            Token::True,
            Token::Comma,
            Token::Word(String::from("key2")),
            Token::Colon,
            Token::False,
            Token::Comma,
            Token::Word(String::from("key3")),
            Token::Colon,
            Token::Null,
            Token::Comma,
            Token::Word(String::from("key4")),
            Token::Colon,
            Token::Word(String::from("value")),
            Token::Comma,
            Token::Word(String::from("key5")),
            Token::Colon,
            Token::Int(String::from("667")),
            Token::RSquirly,
            Token::EOF,
        ];
        let actual_tokens = lexer.read_all_tokens().unwrap();

        println!("expected: {:?}, got: {:?}", expected_tokens, actual_tokens);
        assert_eq!(expected_tokens, actual_tokens);
    }
}
