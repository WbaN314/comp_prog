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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::traverse(&root, i64::MIN, i64::MAX)
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, lower: i64, higher: i64) -> bool {
        if let Some(root) = root {
            let val = root.borrow().val as i64;
            if val > lower && val < higher {
                Self::traverse(&root.borrow().left, lower, val) && Self::traverse(&root.borrow().right, val, higher)
            } else {
                false
            }
        } else {
            true
        }
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