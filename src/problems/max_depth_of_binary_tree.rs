
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