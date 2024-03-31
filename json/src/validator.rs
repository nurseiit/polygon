use crate::{
    fs_utils::read_file_contents,
    json_lexer::{lexer::Lexer, token::Token},
};
use anyhow::Result;

struct JsonValidator {
    lexer: Lexer,
}

impl JsonValidator {
    pub fn new(input: String) -> JsonValidator {
        JsonValidator {
            lexer: Lexer::new(input),
        }
    }

    pub fn is_valid(&mut self) -> bool {
        let all_tokens = self.lexer.read_all_tokens();
        // TODO: validate here
        all_tokens == vec![Token::LSquirly, Token::RSquirly, Token::EOF]
    }
}

pub fn is_file_a_valid_json(path: &std::path::PathBuf) -> Result<bool> {
    let file_contents = read_file_contents(path)?;
    let mut json_validator = JsonValidator::new(file_contents);

    Ok(json_validator.is_valid())
}

#[cfg(test)]
mod validator_tests {
    use super::*;
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
