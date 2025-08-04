use std::cell::RefCell;
use std::rc::Rc;
pub struct TreeNode{
    pub val:i32,
    pub left:Option<Rc<RefCell<TreeNode>>>,
    pub right:Option<Rc<RefCell<TreeNode>>>
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

  pub struct  Solution ;
  impl Solution {
    pub fn solve (root:Option<Rc<RefCell<TreeNode>>>) -> i32{
        Self::max_depth(root)
    }

    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left_depth = Self::max_depth(node.borrow().left.clone());
                let right_depth = Self::max_depth(node.borrow().right.clone());
                1 + std::cmp::max(left_depth, right_depth)
            },
            None => 0
        }
    }
  }