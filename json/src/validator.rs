use crate::{
    fs_utils::read_file_contents,
    json_lexer::{lexer::Lexer, token::Token},
};
use anyhow::Result;

struct JsonValidator {
    tokens: Vec<Token>,
}

impl JsonValidator {
    pub fn new(input: String) -> JsonValidator {
        let tokens = Lexer::new(input).read_all_tokens();
        JsonValidator { tokens }
    }

    pub fn is_valid(&self) -> bool {
        self.is_wrapped_in_squirlies()
    }

    fn is_wrapped_in_squirlies(&self) -> bool {
        if self.tokens.len() >= 2 {
            self.tokens.starts_with(&[Token::LSquirly])
                && self.tokens.ends_with(&[Token::RSquirly, Token::EOF])
        } else {
            false
        }
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

    fn test_step(step: i32, is_valid: bool) {
        let json_path = get_test_file_path_by_step(step, is_valid);
        let want = is_valid;
        let result = is_file_a_valid_json(&json_path).unwrap();
        assert_eq!(want, result);
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
    #[ignore = "not yet implemented"]
    fn step_2_valid_test() {
        test_step(2, true)
    }
}