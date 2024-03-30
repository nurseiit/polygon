use anyhow::{Context, Result};

pub fn read_file_contents(path: &std::path::PathBuf) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("could not find file at '{}'", path.display()))
}

pub fn get_test_file_path_by_step(step: i32, is_valid: bool) -> std::path::PathBuf {
    let file_name = if is_valid { "valid" } else { "invalid" };
    let path_str = format!("teststeps/tests/step{}/{}.json", step, file_name);
    std::path::PathBuf::from(path_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_contents_test() {
        let valid_json_path = get_test_file_path_by_step(1, true);
        let want = "{}";
        let result = read_file_contents(&valid_json_path).unwrap();
        assert_eq!(want, result);
    }
}
