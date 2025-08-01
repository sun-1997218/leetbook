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
  pub fn solve(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    Self::in_order(root, &mut ans);
    ans
  }

  fn in_order(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
    if root.is_some(){
      let node = root.unwrap();
      Self::in_order(node.borrow().left.clone(), ans);
      ans.push(node.borrow().val);
      Self::in_order(node.borrow().right.clone(), ans);
    }
  }
}