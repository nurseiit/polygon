use anyhow::{Context, Result};

pub fn read_file_contents(path: &std::path::PathBuf) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("could not find file at '{}'", path.display()))
}

pub fn get_test_file_path(
    step: i32,
    is_valid: bool,
    test_num: Option<i32>,
    is_full: bool,
) -> std::path::PathBuf {
    if is_full {
        let file_name = if is_valid { "pass" } else { "fail" };
        let path_str = format!("tests/full_tests/{}{}.json", file_name, step);
        std::path::PathBuf::from(path_str)
    } else {
        let basic_file_name = if is_valid { "valid" } else { "invalid" };
        let basic_path_str = format!(
            "tests/basic_tests/step{}/{}{}.json",
            step,
            basic_file_name,
            test_num
                .map(|num| num.to_string())
                .unwrap_or(String::from(""))
        );
        std::path::PathBuf::from(basic_path_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_contents_test() {
        let valid_json_path = get_test_file_path(1, true, None, false);
        let want = "{}";
        let result = read_file_contents(&valid_json_path).unwrap();
        assert_eq!(want, result);
    }
}
