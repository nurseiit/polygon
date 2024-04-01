use crate::{
    fs_utils::read_file_contents,
    json_lexer::{lexer::Lexer, token::Token},
};
use anyhow::Result;

struct JsonValidator {
    tokens: Result<Vec<Token>>,
}

impl JsonValidator {
    pub fn new(input: String) -> JsonValidator {
        let tokens = Lexer::new(input).read_all_tokens();
        JsonValidator { tokens }
    }

    pub fn is_valid(&self) -> bool {
        match &self.tokens {
            Ok(tokens) => {
                JsonValidator::is_wrapped_in_squirlies(tokens)
                    && JsonValidator::are_brackets_balanced(tokens)
            }
            _ => false,
        }
    }

    fn is_wrapped_in_squirlies(tokens: &Vec<Token>) -> bool {
        if tokens.len() >= 2 {
            tokens.starts_with(&[Token::LSquirly])
                && tokens.ends_with(&[Token::RSquirly, Token::EOF])
        } else {
            false
        }
    }

    fn are_brackets_balanced(tokens: &Vec<Token>) -> bool {
        let mut stack = vec![];
        for token in tokens {
            if token.is_open_bracket() {
                stack.push(token);
            }
            if token.is_close_bracket() {
                match stack.pop() {
                    Some(top) => {
                        let open_bracket = token.convert_to_open_bracket();
                        if top != open_bracket {
                            return false;
                        }
                    }
                    None => return false,
                };
            }
        }
        stack.is_empty()
    }
}

pub fn is_file_a_valid_json(path: &std::path::PathBuf) -> Result<bool> {
    let file_contents = read_file_contents(path)?;
    let json_validator = JsonValidator::new(file_contents);

    Ok(json_validator.is_valid())
}

#[cfg(test)]
mod validator_tests {
    use super::is_file_a_valid_json;
    use crate::fs_utils::get_test_file_path_by_step;

    fn test_step_with_num(step: i32, is_valid: bool, test_num: Option<i32>) {
        let json_path = get_test_file_path_by_step(step, is_valid, test_num);
        let want = is_valid;
        let result = is_file_a_valid_json(&json_path).unwrap();
        assert_eq!(want, result);
    }

    fn test_step(step: i32, is_valid: bool) {
        test_step_with_num(step, is_valid, None)
    }

    #[test]
    fn step_1_valid_test() {
        test_step(1, true);
    }

    #[test]
    fn step_1_invalid_test() {
        test_step(1, false);
    }

    #[test]
    fn step_2_valid_test() {
        test_step(2, true)
    }

    #[test]
    fn step_2_invalid_test() {
        test_step(2, false)
    }
}
