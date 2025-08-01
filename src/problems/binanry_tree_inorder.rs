use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub struct Solution;

impl Solution {
  // 递归方法 - 中序遍历
  pub fn solve(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = Vec::new();
    Self::inorder_recursive(root, &mut ans);
    ans
  }

  // 递归辅助函数
  fn inorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
    if let Some(node) = root {
      // 访问左子树
      Self::inorder_recursive(node.borrow().left.clone(), ans);
      // 访问根节点
      ans.push(node.borrow().val);
      // 访问右子树
      Self::inorder_recursive(node.borrow().right.clone(), ans);
    }
  }

  // 迭代方法 - 使用栈
  pub fn solve_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut stack = Vec::new();
    let mut current = root;

    while current.is_some() || !stack.is_empty() {
      // 一直往左走，把所有左节点入栈
      while let Some(node) = current {
        stack.push(node.clone());
        current = node.borrow().left.clone();
      }

      // 弹出栈顶元素
      if let Some(node) = stack.pop() {
        ans.push(node.borrow().val);
        // 转向右子树
        current = node.borrow().right.clone();
      }
    }

    ans
  }
}