mod problems;

use problems::binanry_tree_inorder::{Solution, TreeNode};
use problems::max_depth_of_binary_tree::{Solution as MaxDepthSolution, TreeNode as MaxDepthTreeNode};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 测试中序遍历
    let root = create_sample_tree();
    println!("中序遍历结果:");
    let result = Solution::solve(Some(root));
    println!("递归方法: {:?}", result);
    
    // 测试最大深度
    let max_depth_root = create_max_depth_tree();
    println!("\n最大深度测试:");
    let depth = MaxDepthSolution::solve(Some(max_depth_root));
    println!("二叉树最大深度: {}", depth);
    
    // 演示为什么需要复杂的类型
    demonstrate_ownership_issues();
}

fn create_sample_tree() -> Rc<RefCell<TreeNode>> {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    let right = TreeNode::new(3);
    let left_left = TreeNode::new(4);
    let left_right = TreeNode::new(5);
    
    left.left = Some(Rc::new(RefCell::new(left_left)));
    left.right = Some(Rc::new(RefCell::new(left_right)));
    
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    
    Rc::new(RefCell::new(root))
}

fn create_max_depth_tree() -> Rc<RefCell<MaxDepthTreeNode>> {
    // 创建一个深度为3的二叉树
    //       1
    //      / \
    //     2   3
    //    / \
    //   4   5
    //  /
    // 6
    
    let mut root = MaxDepthTreeNode::new(1);
    let mut left = MaxDepthTreeNode::new(2);
    let right = MaxDepthTreeNode::new(3);
    let mut left_left = MaxDepthTreeNode::new(4);
    let left_right = MaxDepthTreeNode::new(5);
    let left_left_left = MaxDepthTreeNode::new(6);
    
    left_left.left = Some(Rc::new(RefCell::new(left_left_left)));
    left.left = Some(Rc::new(RefCell::new(left_left)));
    left.right = Some(Rc::new(RefCell::new(left_right)));
    
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    
    Rc::new(RefCell::new(root))
}
