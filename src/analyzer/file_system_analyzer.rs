use std::fs;
use std::path::PathBuf;

pub fn analyze(path: PathBuf) -> u64 {
    let metadata = fs::metadata(&path).unwrap();
    return metadata.len();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use rstest::rstest;
    use tempfile::{NamedTempFile};
    use crate::analyzer::file_system_analyzer::analyze;

    #[rstest]
    #[case("Hello", 5)]
    #[case("", 0)]
    fn analyze_small_file(#[case] content: &str, #[case] expected: u64) {
        // Arrange
        let file = NamedTempFile::new().unwrap();

        // Act
        fs::write(&file.path(), content).unwrap();

        // Assert
        let result = analyze(file.path().to_path_buf());
        assert_eq!(result, expected);
    }
}