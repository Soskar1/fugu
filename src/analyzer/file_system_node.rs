use std::fs;
use std::path::PathBuf;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum NodeType {
    File,
    Directory
}

pub struct FileSystemNode {
    node_name: String,
    size: u64,
    node_type: NodeType,
    nodes: Vec<FileSystemNode>
}

impl FileSystemNode {
    pub fn new(name: &str, size: u64, node_type: NodeType) -> FileSystemNode {
        FileSystemNode {
            node_name: name.to_string(),
            size,
            node_type,
            nodes: Vec::new()
        }
    }
    pub fn node_name(&self) -> &str { &self.node_name }
    pub fn size(&self) -> u64 { self.size }
    pub fn node_type(&self) -> NodeType { self.node_type }
    pub fn add(&mut self, node: FileSystemNode) {

    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::*;

    #[test]
    fn add_adds_node() {
        // Arrange
        let mut node = FileSystemNode::new("directory", 1, NodeType::Directory);

        // Act
        node.add(FileSystemNode::new("directory", 1, NodeType::File));

        // Assert
        assert_eq!(node.nodes.len(), 1);
        assert_eq!(node.nodes[0].node_name(), "directory");
        assert_eq!(node.nodes[0].node_type(), NodeType::File);
        assert_eq!(node.nodes[0].size(), 1);
    }

    #[rstest]
    #[should_panic]
    #[case(NodeType::Directory)]
    #[case(NodeType::File)]
    fn add_does_not_add_file_to_file(#[case] node_type: NodeType) {
        // Arrange
        let mut node = FileSystemNode::new("file", 1, NodeType::File);

        // Act & Assert
        node.add(FileSystemNode::new("file", 1, node_type));
    }
}