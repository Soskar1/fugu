use std::fs;
use assert_cmd::Command;
use tempfile::{tempdir};

const FUGU: &str = "fugu";

#[test]
fn fugu_prints_file_name_and_size_in_bytes() {
    // Arrange
    let temp= tempdir().unwrap();
    let root_path = temp.path().join("root");
    let _ = fs::create_dir(&root_path);
    
    let file = root_path.join("hello.txt");
    let _ = fs::write(&file, "Hello, World!");

    let mut command = Command::cargo_bin(FUGU).unwrap();

    // Act & Assert
    command
        .arg(root_path)
        .assert()
        .success()
        .stdout("./root 13B\n\thello.txt 13B\n");
}

#[test]
fn fugu_accepts_working_directory() {
    // Arrange
    let temp= tempdir().unwrap();
    let root_path = temp.path().join("root");
    let _ = fs::create_dir(&root_path);

    let file = root_path.join("hello.txt");
    let _ = fs::write(&file, "Hello, World!");

    let mut command = Command::cargo_bin(FUGU).unwrap();

    // Act & Assert
    command
        .current_dir(root_path)
        .arg("./")
        .assert()
        .success()
        .stdout("./ 13B\n\thello.txt 13B\n");
}