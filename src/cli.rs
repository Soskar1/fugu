use std::path::PathBuf;
use clap::Parser;
use fugu::analyzer::file_system_analyzer::analyze;

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
    println!("./{} {}B", root_node.node_name(), root_node.size());
    
    for node in root_node.iter() {
        println!("\t{} {}B", node.node_name(), node.size())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
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
}