pub trait PrintableTreeNode {
    fn data(&self) -> &str;

    fn tree_nodes(&self) -> &[Self] where Self:Sized;
}