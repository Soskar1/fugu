use std::fs;
use assert_cmd::Command;
use tempfile::{tempdir, NamedTempFile};

const FUGU: &str = "fugu";

#[test]
fn fugu_prints_file_name_and_size_in_bytes() {
    // Arrange
    let directory = tempdir().unwrap();
    let file = directory.path().join("hello.txt");

    fs::write(&file, "Hello, World!").unwrap();

    let mut command = Command::cargo_bin(FUGU).unwrap();

    // Act & Assert
    command
        .arg(&file)
        .assert()
        .success()
        .stdout("hello.txt   13B\n");
}