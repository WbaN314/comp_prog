// https://leetcode.com/problems
pub struct Solution {}

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut inorder: Vec<i32> = Vec::new();

        match root {
            None => return inorder,
            Some(node) => {
                if let Some(left_node) = &node.borrow().left {
                    inorder.append(&mut Solution::inorder_traversal(Some(Rc::clone(left_node))));
                }
                inorder.push(node.borrow().val);
                if let Some(right_node) = &node.borrow().right {
                    inorder.append(&mut Solution::inorder_traversal(Some(Rc::clone(right_node))));
                }
                return inorder
            }
        };
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