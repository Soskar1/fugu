pub trait PrintableTreeNode {
    fn get_data(&self) -> &str;

    fn get_tree_nodes(&self) -> &[Self] where Self:Sized;
}