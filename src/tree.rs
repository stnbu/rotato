use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::TAU;

#[derive(Debug, Default)]
pub struct TreeNode {
    pub transform: Transform,
    pub rps: f32,
    pub children: Vec<TreeNode>,
}

pub fn generate_tree(depth: i32, max_depth: i32, max_children: usize) -> TreeNode {
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
    let x: f32 = 1.0 + rng.gen::<f32>() * 0.2;
    let y: f32 = 1.0 + rng.gen::<f32>() * 0.1;
    let transform = Transform::from_translation(Vec3::Y * x + Vec3::X * x);
    let rps: f32 = rng.gen::<f32>() * 0.1 * TAU;
    TreeNode {
        transform,
        rps,
        ..Default::default()
    }
    .apply_children(children)
}

impl TreeNode {
    pub fn apply_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
}
