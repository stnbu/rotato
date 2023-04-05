use std::collections::HashMap;

#[derive(Debug, Default)]
struct TreeNode {
    transform: bool,
    rotation: bool,
    children: Vec<TreeNode>,
}

use rand::prelude::*;

fn generate_tree(depth: i32, max_depth: i32, max_children: usize) -> TreeNode {
    if depth >= max_depth {
        return TreeNode::default();
    }
    let mut rng = rand::thread_rng();
    let num_children: usize = rng.gen::<usize>() % max_children + 1;
    let mut children = vec![];
    for _ in 0..num_children {
        print!(".");
        children.push(generate_tree(depth + 1, max_depth, max_children));
    }
    TreeNode::default().apply_children(children)
}

impl TreeNode {
    fn apply_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
}

pub fn blah() {
    let depth = 4;
    let max_children = 3;

    let root = generate_tree(0, depth, max_children);

    //let hash_map = walk_tree(&root);
    //println!("{:#?}", hash_map);
}
