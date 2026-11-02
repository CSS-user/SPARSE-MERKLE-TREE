use crate::node::Node;
pub struct SparseMerkleTree {
    root: [u8; 32],
    depth: usize,
    members: Vec<Node>
}

impl SparseMerkleTree {
    pub fn new(depth: usize) -> Self {
        SparseMerkleTree { root: [0; 32], depth, members: Vec::new() }
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

use super::* ;

    #[test]
    fn test_tree_initialization() {
        let tree = SparseMerkleTree::new(4);

        assert!(tree.depth == 4);
        assert_eq!(tree.root, [0; 32]);
        assert_eq!(tree.members.len(), 0);
    }
}