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
                JsonValidator::is_wrapped_in_correct_brackets(tokens)
                    && JsonValidator::are_brackets_balanced(tokens)
                    && JsonValidator::no_trailing_commas(tokens)
                    && JsonValidator::no_leading_zeroes_in_numbers(tokens)
            }
            _ => false,
        }
    }

    fn no_leading_zeroes_in_numbers(tokens: &[Token]) -> bool {
        tokens.iter().all(|token| match token {
            Token::Int(x) => !x.starts_with('0') || x.len() == 1,
            _ => true,
        })
    }

    fn no_trailing_commas(tokens: &Vec<Token>) -> bool {
        for i in 1..tokens.len() {
            let current = tokens.get(i).unwrap();
            let prev = tokens.get(i - 1).unwrap();
            if prev.is_comma() && current.is_close_bracket() {
                return false;
            }
        }
        true
    }

    fn is_wrapped_in_correct_brackets(tokens: &Vec<Token>) -> bool {
        if tokens.len() >= 2 {
            tokens.starts_with(&[Token::LSquirly])
                && tokens.ends_with(&[Token::RSquirly, Token::EOF])
                || tokens.starts_with(&[Token::LSquare])
                    && tokens.ends_with(&[Token::RSquare, Token::EOF])
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
    use crate::fs_utils::get_test_file_path;

    fn test_step_with_optional_num(
        step: i32,
        is_valid: bool,
        test_num: Option<i32>,
        is_full: bool,
    ) {
        let json_path = get_test_file_path(step, is_valid, test_num, is_full);
        let want = is_valid;
        let result = is_file_a_valid_json(&json_path).unwrap();
        assert_eq!(want, result);
    }

    fn test_step_with_num(step: i32, is_valid: bool, test_num: i32) {
        test_step_with_optional_num(step, is_valid, Some(test_num), false)
    }

    fn test_step(step: i32, is_valid: bool) {
        test_step_with_optional_num(step, is_valid, None, false)
    }

    fn test_full(step: i32, is_valid: bool) {
        test_step_with_optional_num(step, is_valid, None, true)
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

    #[test]
    fn steps_2_valid2_test() {
        test_step_with_num(2, true, 2)
    }

    #[test]
    fn step_2_invalid2_test() {
        test_step_with_num(2, false, 2)
    }

    #[test]
    fn step_3_valid_test() {
        test_step(3, true)
    }

    #[test]
    fn step_3_invalid_test() {
        test_step(3, false)
    }

    #[test]
    fn step_4_invalid_test() {
        test_step(4, false)
    }

    #[test]
    fn step_4_valid_test() {
        test_step(4, true)
    }

    #[test]
    fn steps_4_valid2_test() {
        test_step_with_num(4, true, 2)
    }

    #[test]
    fn full_test_fail() {
        for i in 1..34 {
            println!("testing fail{}.json", i);
            test_full(i, false)
        }
    }
}
