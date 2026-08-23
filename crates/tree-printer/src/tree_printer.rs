use crate::printable_tree_node::PrintableTreeNode;

#[derive(Debug, Clone, Copy)]
pub struct TreeFormatOptions {
    indentation_size: usize
}

impl TreeFormatOptions {
    pub fn new(indentation_size: usize) -> TreeFormatOptions {
        if indentation_size == 0 {
            panic!("indentation cannot be zero!");
        }
        
        TreeFormatOptions { 
            indentation_size
        }
    }
}

pub fn get_tree_string(root_node: &impl PrintableTreeNode) -> String {
    let options = TreeFormatOptions::new(4);
    
    get_tree_string_with_options(root_node, options)
}

pub fn get_tree_string_with_options(root_node: &impl PrintableTreeNode, options: TreeFormatOptions) -> String {
    let mut output = root_node.get_data().to_string();

    let indentation = if options.indentation_size == 1 {
        String::new()
    } else {
        format!("{} ", "─".repeat(options.indentation_size - 2))
    };

    let tree = get_tree_string_internal(root_node, options, &indentation, "");
    output.push_str(&tree);

    output
}

fn get_tree_string_internal(root_node: &impl PrintableTreeNode, options: TreeFormatOptions, indentation: &str, prefix: &str) -> String {
    let mut output = "".to_string();
    let mut nodes = root_node.get_tree_nodes().iter().peekable();

    while let Some(node) = nodes.next() {
        let is_last  = nodes.peek().is_none();

        let line = if is_last {
            format!("\n{}└{}{}", prefix, indentation, node.get_data())
        } else {
            format!("\n{}├{}{}", prefix, indentation, node.get_data())
        };

        output.push_str(&line);

        if !node.get_tree_nodes().is_empty() {
            let prefix = if is_last {
                format!("{}{}", prefix, " ".repeat(options.indentation_size))
            } else {
                format!("{}│{}", prefix, " ".repeat(options.indentation_size - 1))
            };

            let sub_tree = get_tree_string_internal(node, options, indentation, &prefix);
            output.push_str(&sub_tree);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use std::vec;
    use rstest::rstest;
    use super::*;

    struct TestTreeNode {
        data: String,
        children: Vec<TestTreeNode>
    }

    impl TestTreeNode {
        fn new(data: &str, children: Vec<TestTreeNode>) -> TestTreeNode {
            TestTreeNode {
                data: data.to_string(),
                children
            }
        }
    }

    impl PrintableTreeNode for TestTreeNode {
        fn get_data(&self) -> &str {
            &self.data
        }

        fn get_tree_nodes(&self) -> &[Self]{
            &self.children
        }
    }

    #[test]
    fn get_tree_string_returns_root() {
        // Arrange
        let root = TestTreeNode::new("root", vec![]);

        // Act
        let result = get_tree_string(&root);

        // Assert
        assert_eq!(result, "root");
    }

    #[test]
    fn get_tree_string_with_options_inserts_leaf_connector_character() {
        // Arrange
        let root = TestTreeNode::new("root", vec![
            TestTreeNode::new("depth1", vec![])
        ]);

        let options = TreeFormatOptions::new(2);

        // Act
        let result = get_tree_string_with_options(&root, options);

        // Assert
        assert_eq!(result, "root\n└ depth1");
    }

    #[test]
    fn get_tree_string_with_options_inserts_branch_connector_character() {
        // Arrange
        let root = TestTreeNode::new("root", vec![
            TestTreeNode::new("depth1", vec![]),
            TestTreeNode::new("depth1", vec![])
        ]);

        let options = TreeFormatOptions::new(2);

        // Act
        let result = get_tree_string_with_options(&root, options);

        // Assert
        assert_eq!(result, "root\n├ depth1\n└ depth1");
    }

    #[test]
    fn get_tree_string_inserts_horizontal_connector_character() {
        // Arrange
        let root = TestTreeNode::new("root", vec![
            TestTreeNode::new("depth1", vec![]),
            TestTreeNode::new("depth1", vec![])
        ]);

        // Act
        let result = get_tree_string(&root);

        // Assert
        assert_eq!(result, "root
├── depth1
└── depth1");
    }

    #[rstest]
    #[case(1, "root\n└depth1")]
    #[case(2, "root\n└ depth1")]
    #[case(3, "root\n└─ depth1")]
    #[case(4, "root\n└── depth1")]
    #[case(5, "root\n└─── depth1")]
    fn get_tree_string_with_options_indentation(#[case] indentation_size: usize, #[case] expected: &str) {
        // Arrange
        let root = TestTreeNode::new("root", vec![
            TestTreeNode::new("depth1", vec![])
        ]);

        let options = TreeFormatOptions::new(indentation_size);

        // Act
        let result = get_tree_string_with_options(&root, options);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn tree_format_options_new_panics_when_zero_indenation_size() {
        // Arragne & Act & Assert
        TreeFormatOptions::new(0);
    }

    #[test]
    fn get_tree_string_inserts_vertical_connector_character() {
        // Arrange
        let root = TestTreeNode::new("root", vec![
            TestTreeNode::new("depth1", vec![
                TestTreeNode::new("depth2", vec![])
            ]),
            TestTreeNode::new("depth1", vec![])
        ]);

        // Act
        let result = get_tree_string(&root);

        // Assert
        assert_eq!(result, "root
├── depth1
│   └── depth2
└── depth1");
    }

    #[test]
    fn get_tree_string_vertical_lines_on_demand() {
        // Arrange
        let root = TestTreeNode::new("root", vec![
            TestTreeNode::new("depth1", vec![
                TestTreeNode::new("depth2", vec![
                    TestTreeNode::new("depth3", vec![
                        TestTreeNode::new("depth4", vec![])
                    ])
                ]),
                TestTreeNode::new("depth2", vec![])
            ]),
            TestTreeNode::new("depth1", vec![])
        ]);

        // Act
        let result = get_tree_string(&root);

        // Assert
        assert_eq!(result, "root
├── depth1
│   ├── depth2
│   │   └── depth3
│   │       └── depth4
│   └── depth2
└── depth1");
    }

    #[test]
    fn get_tree_string_is_recursive() {
        // Arrange
        let root = TestTreeNode::new("root", vec![
            TestTreeNode::new("depth1", vec![
                TestTreeNode::new("depth2", vec![
                    TestTreeNode::new("depth3", vec![
                        TestTreeNode::new("depth4", vec![])
                    ])
                ])
            ])
        ]);

        // Act
        let result = get_tree_string(&root);

        // Assert
        assert_eq!(result, "root
└── depth1
    └── depth2
        └── depth3
            └── depth4");
    }
}