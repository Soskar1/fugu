use std::{fs, path::PathBuf};
use assert_cmd::Command;
use tempfile::{TempDir, tempdir};

const FUGU: &str = "fugu-cli";

fn create_root_folder() -> (TempDir, PathBuf) {
    let temp= tempdir().unwrap();
    let root_path = temp.path().join("root");
    let _ = fs::create_dir(&root_path);

    (temp, root_path)
}

#[test]
fn fugu_prints_file_name_and_size_in_bytes() {
    // Arrange
    let (_temp, root_path) = create_root_folder();
    
    let file = root_path.join("hello.txt");
    let _ = fs::write(&file, "Hello, World!");

    let mut command = Command::cargo_bin(FUGU).unwrap();

    // Act & Assert
    command
        .arg(root_path)
        .assert()
        .success()
        .stdout("./root 13B\n└── hello.txt 13B\n");
}

#[test]
fn fugu_accepts_working_directory() {
    // Arrange
    let (_temp, root_path) = create_root_folder();

    let file = root_path.join("hello.txt");
    let _ = fs::write(&file, "Hello, World!");

    let mut command = Command::cargo_bin(FUGU).unwrap();

    // Act & Assert
    command
        .current_dir(root_path)
        .arg("./")
        .assert()
        .success()
        .stdout("./ 13B\n└── hello.txt 13B\n");
}

#[test]
fn fugu_prints_kilobytes() {
    // Arrange
    let (_temp, root_path) = create_root_folder();

    let file = root_path.join("kb_test.txt");
    fs::write(&file, vec![0u8; 1000]).unwrap();

    let mut command = Command::cargo_bin(FUGU).unwrap();

    // Act & Assert
    command
        .current_dir(root_path)
        .arg("./")
        .assert()
        .success()
        .stdout("./ 1KB\n└── kb_test.txt 1KB\n");
}

#[test]
fn fugu_prints_info_recursively() {
   // Arrange
    let (_temp, root_path) = create_root_folder();

    fs::create_dir(&root_path.join("test")).unwrap();
    fs::write(&root_path.join("test/test.txt"), "aaa").unwrap();

    let mut command = Command::cargo_bin(FUGU).unwrap();

    // Act & Assert
    command
        .current_dir(root_path)
        .arg("./")
        .assert()
        .success()
        .stdout("./ 3B\n└── ./test 3B\n    └── test.txt 3B\n");
}