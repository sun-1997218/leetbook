use std::rc::Rc;
use std::cell::RefCell;
pub struct TreeNode{
    pub val:i32,
    pub left:Option<Rc<RefCell<TreeNode>>>,
    pub right:Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    #[inline]
    pub fn new(val:i32) -> Self {
        TreeNode{val,left:None,right:None}
    }
}

pub struct  Solution;
impl Solution {
    pub fn solve (root: Option<Rc<RefCell<TreeNode>>>)->Option<Rc<RefCell<TreeNode>>>{
        if let Some(node) = root.clone() {
            
            let left = Self::solve(node.borrow_mut().left.clone());
            let right = Self::solve(node.borrow_mut().right.clone());
            
           
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
        root
    }
}
