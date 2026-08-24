#[derive(PartialEq, Debug, Copy, Clone)]
pub enum NodeType {
    File,
    Directory
}

#[derive(Debug, PartialEq)]
pub struct FileSystemNode {
    node_name: String,
    size: u64,
    node_type: NodeType,
    nodes: Vec<FileSystemNode>
}

impl FileSystemNode {
    fn new_internal(name: &str, size: u64, node_type: NodeType, nodes: Vec<FileSystemNode>) -> FileSystemNode {
        FileSystemNode {
            node_name: name.to_string(),
            size,
            node_type,
            nodes
        }
    }

    pub fn new_file(name: &str, size: u64) -> FileSystemNode {
        FileSystemNode::new_internal(name, size, NodeType::File, vec![])
    }

    pub fn new_directory(name: &str, size: u64, nodes: Vec<FileSystemNode>) -> FileSystemNode {
        FileSystemNode::new_internal(name, size, NodeType::Directory, nodes)
    }

    pub fn node_name(&self) -> &str { &self.node_name }
    pub fn size(&self) -> u64 { self.size }
    pub fn node_type(&self) -> NodeType { self.node_type }

    pub fn iter(&self) -> impl Iterator<Item = &FileSystemNode> {
        self.nodes.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_directory_creates_directory() {
        // Arrange & Act
        let node = FileSystemNode::new_directory("directory", 0, vec![]);

        // Assert
        assert_eq!(node.node_type, NodeType::Directory);
    }

    #[test]
    fn new_file_creates_file() {
        // Arrange & Act
        let node = FileSystemNode::new_file("file", 1);

        // Assert
        assert_eq!(node.node_type, NodeType::File);
    }

    #[test]
    fn iter_returns_nodes_iterator() {
        // Arrange
        let node = FileSystemNode::new_directory("test", 0, vec![
            FileSystemNode::new_file("name1", 1),
            FileSystemNode::new_file("name2", 1),
            FileSystemNode::new_directory("name3", 1, vec![]),
        ]);

        // Act
        let mut iterator = node.iter();

        // Assert
        assert_eq!(Some(&FileSystemNode::new_file("name1", 1)), iterator.next());
        assert_eq!(Some(&FileSystemNode::new_file("name2", 1)), iterator.next());
        assert_eq!(Some(&FileSystemNode::new_directory("name3", 1, vec![])), iterator.next());
    }
}