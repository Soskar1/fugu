use std::fs;
use std::path::PathBuf;
use crate::analyzer::file_system_node::{FileSystemNode, NodeType};

pub fn analyze(path: PathBuf) -> FileSystemNode {
    let metadata = fs::metadata(&path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let node_type = if metadata.is_dir() {
        NodeType::Directory
    } else {
        NodeType::File
    };

    let mut size = if node_type == NodeType::Directory {
        0
    } else {
        metadata.len()
    };
    
    if node_type == NodeType::Directory {
        let mut nodes: Vec<FileSystemNode> = vec![];

        for entry in fs::read_dir(&path).unwrap() {
            let subfile_node = analyze(entry.unwrap().path().to_path_buf());
            size += subfile_node.size();

            nodes.push(subfile_node);
        }

        FileSystemNode::new_directory(file_name, size, nodes)
    } else {
        FileSystemNode::new_file(file_name, size)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use rstest::rstest;
    use tempfile::{tempdir, NamedTempFile};
    use crate::analyzer::file_system_analyzer::analyze;
    use crate::analyzer::file_system_node::{FileSystemNode, NodeType};

    #[rstest]
    #[case("Hello", 5)]
    #[case("", 0)]
    fn analyze_small_file(#[case] content: &str, #[case] expected: u64) {
        // Arrange
        let file = NamedTempFile::new().unwrap();
        fs::write(&file.path(), content).unwrap();

        // Act
        let node = analyze(file.path().to_path_buf());

        // Assert
        assert_eq!(node.node_type(), NodeType::File);
        assert_eq!(node.size(), expected);
    }

    #[test]
    fn analyze_empty_directory() {
        // Arrange
        let directory = tempdir().unwrap();

        // Act
        let node = analyze(directory.path().to_path_buf());

        // Assert
        assert_eq!(node.size(), 0);
    }

    #[test]
    fn analyze_directory_with_files() {
        // Arrange
        let directory = tempdir().unwrap();
        let _ = NamedTempFile::new_in(directory.path()).unwrap();
        let file_with_contents = NamedTempFile::new_in(directory.path()).unwrap();

        fs::write(&file_with_contents.path(), "Hello, World!").unwrap();

        // Act
        let node = analyze(directory.path().to_path_buf());

        // Assert
        assert_eq!(node.node_type(), NodeType::Directory);
        assert_eq!(node.size(), 13);
    }

    #[test]
    fn analyze_deep_recursion() {
        // Arrange
        let directory = tempdir().unwrap();
        fs::create_dir(directory.path().join("sub")).unwrap();
        fs::create_dir(directory.path().join("sub/sub")).unwrap();
        fs::create_dir(directory.path().join("sub/sub/subdir")).unwrap();
        fs::write(
            directory.path().join("sub/sub/subdir/hello.txt"),
            "Hello!"
        ).unwrap();

        // Act
        let node = analyze(directory.path().to_path_buf());

        // Assert
        assert_eq!(node.size(), 6);
    }

    #[test]
    fn analyze_preserves_file_system_structure() {
        // Arrange
        let directory = tempdir().unwrap();
        fs::write(directory.path().join("a.txt"), "").unwrap();
        
        fs::create_dir(directory.path().join("sub")).unwrap();
        fs::write(directory.path().join("sub/b.txt"), "").unwrap();
        fs::write(directory.path().join("sub/c.txt"), "").unwrap();
        fs::write(directory.path().join("sub/d.txt"), "").unwrap();
        
        fs::create_dir(directory.path().join("sub/sub")).unwrap();
        fs::write(directory.path().join("sub/sub/e.txt"), "").unwrap();
        fs::write(directory.path().join("sub/sub/f.txt"), "").unwrap();

        // Act
        let node = analyze(directory.path().to_path_buf());

        // Assert
        assert_eq!(node.node_type(), NodeType::Directory);
        assert_eq!(node.size(), 0);
        
        let item = node.iter().find(|&x| x.node_name() == "a.txt");
        assert_eq!(Some(&FileSystemNode::new_file("a.txt", 0)), item);

        let item = node.iter().find(|&x| x.node_name() == "e.txt");
        assert_eq!(None, item);
        
        let sub_directory = node.iter().find(|&x| x.node_name() == "sub").unwrap();
        assert_eq!(sub_directory.size(), 0);
        assert_eq!(sub_directory.node_type(), NodeType::Directory);

        let item = sub_directory.iter().find(|&x| x.node_name() == "b.txt");
        assert_eq!(Some(&FileSystemNode::new_file("b.txt", 0)), item);
        let item = sub_directory.iter().find(|&x| x.node_name() == "c.txt");
        assert_eq!(Some(&FileSystemNode::new_file("c.txt", 0)), item);
        let item = sub_directory.iter().find(|&x| x.node_name() == "d.txt");
        assert_eq!(Some(&FileSystemNode::new_file("d.txt", 0)), item);
        let item = sub_directory.iter().find(|&x| x.node_name() == "e.txt");
        assert_eq!(None, item);
        let item = sub_directory.iter().find(|&x| x.node_name() == "a.txt");
        assert_eq!(None, item);

        let sub_directory = sub_directory.iter().find(|&x| x.node_name() == "sub").unwrap();
        assert_eq!(sub_directory.size(), 0);
        assert_eq!(sub_directory.node_type(), NodeType::Directory);

        let item = sub_directory.iter().find(|&x| x.node_name() == "e.txt");
        assert_eq!(Some(&FileSystemNode::new_file("e.txt", 0)), item);
        let item = sub_directory.iter().find(|&x| x.node_name() == "f.txt");
        assert_eq!(Some(&FileSystemNode::new_file("f.txt", 0)), item);
        let item = sub_directory.iter().find(|&x| x.node_name() == "a.txt");
        assert_eq!(None, item);
    }
}