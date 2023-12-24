// https://leetcode.com/problems
pub struct Solution {}

// Definition for a binary tree node.
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

// START SUBMISSION CODE

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
          fn helper(
    l: &Option<Rc<RefCell<TreeNode>>>,
    r: &Option<Rc<RefCell<TreeNode>>>,
  ) -> bool {
    match (l, r) {
      (Some(ref l), Some(ref r)) => {
        l.as_ref().borrow().val == r.as_ref().borrow().val
          && helper(&l.borrow().left, &r.borrow().right)
          && helper(&r.borrow().left, &l.borrow().right)
      }
      (None, None) => true,
      _ => false,
    }
  }
  helper(&root, &root)

    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
     }
 }