use std::cell::RefCell;
use std::rc::Rc;
pub struct TreeNode{
    val:i32,
    left:Option<Rc<RefCell<TreeNode>>>,
    right:Option<Rc<RefCell<TreeNode>>>
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
        let result = self.mathod(root,0);

        result
    }

    pub fn mathod (root:Option<Rc<RefCell<TreeNode>>>,val:i32) ->i32{

        if root.is_some() {
            
            if let Some(left) = root.left{
                let left_val = self.mathod(left,val+1);
                let right_val = self.mathod(root.right,val+1);
                return left_val.max(right_val);
            }
        }

        val
    }


  }