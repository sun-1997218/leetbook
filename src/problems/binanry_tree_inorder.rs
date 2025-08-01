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
  pub fn solve (root:Option<Rc<RefCell<TreeNode>>>) ->Vec<i32>{
      let ans:Vec<i32> = Vec::new();
       
       while(root){
        if(root)
       }


    ans

  }
}