mod problems;

use problems::binanry_tree_inorder::{Solution, TreeNode};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 创建一个简单的二叉树
    //       1
    //      / \
    //     2   3
    //    / \
    //   4   5
    
    let root = create_sample_tree();
    
    println!("中序遍历结果:");
    let result = Solution::solve(Some(root));
    println!("递归方法: {:?}", result);
    
    let root2 = create_sample_tree();
    let result2 = Solution::solve_iterative(Some(root2));
    println!("迭代方法: {:?}", result2);
    
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

fn demonstrate_ownership_issues() {
    println!("\n=== 演示所有权问题 ===");
    
    // 问题1: 如果不用Rc，无法创建循环引用
    // 这会导致编译错误
    /*
    let mut node1 = TreeNode::new(1);
    let mut node2 = TreeNode::new(2);
    
    // 这行会报错，因为node2的所有权被移动了
    node1.left = Some(Box::new(node2));
    node2.right = Some(Box::new(node1)); // 错误！node2已经被移动
    */
    
    // 解决方案: 使用Rc<RefCell<T>>
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    
    // 现在可以安全地创建引用
    node1.borrow_mut().left = Some(node2.clone());
    node2.borrow_mut().right = Some(node1.clone());
    
    println!("成功创建了循环引用!");
    println!("node1的值: {}", node1.borrow().val);
    println!("node2的值: {}", node2.borrow().val);
    
    // 问题2: 如果不用RefCell，无法修改不可变引用
    let node = Rc::new(RefCell::new(TreeNode::new(10)));
    
    // 即使node是不可变的，我们仍然可以修改其内容
    node.borrow_mut().val = 20;
    println!("修改后的值: {}", node.borrow().val);
}
