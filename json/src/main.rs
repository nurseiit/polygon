use anyhow::{Context, Result};
use clap::Parser;

fn read_file_contents(path: &std::path::PathBuf) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("could not find file at '{}'", path.display()))
}

fn is_string_a_valid_json(str: String) -> bool {
    str == "{}"
}

fn is_file_a_valid_json(path: &std::path::PathBuf) -> Result<bool> {
    let file_contents = read_file_contents(path)?;
    let result = is_string_a_valid_json(file_contents);

    Ok(result)
}

#[derive(Parser)]
struct JsonParserCliArgs {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = JsonParserCliArgs::parse();
    let path = args.path;

    if is_file_a_valid_json(&path).is_ok_and(|is_valid| is_valid) {
        println!("'{}' is a VALID json", path.display());
    } else {
        println!("'{}' is NOT a VALID json", path.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_file_path_by_step(step: i32, is_valid: bool) -> std::path::PathBuf {
        let file_name = if is_valid { "valid" } else { "invalid" };
        let path_str = format!("teststeps/tests/step{}/{}.json", step, file_name);
        std::path::PathBuf::from(path_str)
    }

    fn test_step(step: i32, is_valid: bool) {
        let json_path = get_test_file_path_by_step(step, is_valid);
        let want = is_valid;
        let result = is_file_a_valid_json(&json_path).unwrap();
        assert_eq!(want, result);
    }

    #[test]
    fn read_file_contents_test() {
        let valid_json_path = get_test_file_path_by_step(1, true);
        let want = "{}";
        let result = read_file_contents(&valid_json_path).unwrap();
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
    fn step_2_valid_test() {
        test_step(2, true)
    }
}
