// Defineing  a binary tree node structure usingng this line
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
   
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    
    let mut root = TreeNode::new(3);
    let mut left = TreeNode::new(9);
    let right = TreeNode::new(20);
    let mut right_left = TreeNode::new(15);
    let right_right = TreeNode::new(7);

    right.left = Some(Box::new(right_left));
    right.right = Some(Box::new(right_right));

    left.left = None;
    left.right = None;

    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));

    
    let depth = max_depth(Some(Box::new(root)));
    println!("Maximum depth of the binary tree: {}", depth);
}
