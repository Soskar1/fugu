use std::path::PathBuf;
use clap::Parser;
use fugu::analyzer::file_system_analyzer::analyze;
use fugu::analyzer::file_system_node::FileSystemNode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    path: PathBuf
}

pub fn run() {
    let args = Args::parse();
    run_internal(args);
}

fn run_internal(args: Args) {
    if args.path.is_file() {
        panic!("File paths are not allowed");
    }

    let root_node = analyze(args.path);
    let output = node_to_string(&root_node);
    println!("./{output}");
    
    for node in root_node.iter() {
        let output = node_to_string(&node);
        println!("\t{output}")
    }
}

fn node_to_string(node: &FileSystemNode) -> String {
    let size = node.size() as f64;
    let node_name = node.node_name();

    let (size, byte_str) = match size {
        x if x >= 1_000_000_000.0 => (x / 1_000_000_000.0, "GB"),
        x if x >= 1_000_000.0 => (x / 1_000_000.0, "MB"),
        x if x >= 1000.0 => (x / 1000.0, "KB"),
        x => (x, "B")
    };
    
    if size.fract() == 0.0 {
        format!("{node_name} {size}{byte_str}")
    } else {
        let size = (size * 100.0).trunc() / 100.0;
        format!("{node_name} {size:.2}{byte_str}")
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use rstest::rstest;
    use tempfile::NamedTempFile;
    use super::*;

    #[test]
    #[should_panic]
    fn panic_on_file() {
        // Arrange
        let file = NamedTempFile::new().unwrap();
        let args = Args {
            path: PathBuf::from(file.path())
        };

        // Act & Assert
        run_internal(args);
    }

    #[rstest]
    #[case(1, "test.txt 1B")]
    #[case(999, "test.txt 999B")]
    #[case(1000, "test.txt 1KB")]
    #[case(999_999, "test.txt 999.99KB")]
    #[case(1_000_000, "test.txt 1MB")]
    #[case(999_999_999, "test.txt 999.99MB")]
    #[case(1_000_000_000, "test.txt 1GB")]
    fn node_to_string_returns_formatted_string(#[case] size: u64, #[case] expected_string: &str) {
        // Arrange
        let node = FileSystemNode::new_file("test.txt", size);

        // Act
        let formatted_string = node_to_string(&node);

        // Assert
        assert_eq!(formatted_string, expected_string);
    }
}