use std::fs;
use std::path::PathBuf;

pub fn analyze(path: PathBuf) -> u64 {
    let metadata = fs::metadata(&path).unwrap();
    let mut size = metadata.len();
    
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            size += analyze(entry.unwrap().path().to_path_buf());
        }
    }
    
    return size;
}

#[cfg(test)]
mod tests {
    use std::fs;
    use rstest::rstest;
    use tempfile::{tempdir, NamedTempFile};
    use crate::analyzer::file_system_analyzer::analyze;

    #[rstest]
    #[case("Hello", 5)]
    #[case("", 0)]
    fn analyze_small_file(#[case] content: &str, #[case] expected: u64) {
        // Arrange
        let file = NamedTempFile::new().unwrap();
        fs::write(&file.path(), content).unwrap();

        // Act
        let result = analyze(file.path().to_path_buf());

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn analyze_empty_directory() {
        // Arrange
        let directory = tempdir().unwrap();

        // Act
        let result = analyze(directory.path().to_path_buf());

        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn analyze_directory_with_files() {
        // Arrange
        let directory = tempdir().unwrap();
        let _ = NamedTempFile::new_in(directory.path()).unwrap();
        let file_with_contents = NamedTempFile::new_in(directory.path()).unwrap();

        fs::write(&file_with_contents.path(), "Hello, World!").unwrap();

        // Act
        let result = analyze(directory.path().to_path_buf());

        // Assert
        assert_eq!(result, 13);
    }
}