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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match p {
            Some(p_rc) => {
                if let Some(q_rc) = q {
                    if q_rc.borrow().val == p_rc.borrow().val {
                        let  q_left: Option<Rc<RefCell<TreeNode>>>;
                        let  q_right: Option<Rc<RefCell<TreeNode>>>;
                        let  p_left: Option<Rc<RefCell<TreeNode>>>;
                        let  p_right: Option<Rc<RefCell<TreeNode>>>;

                        q_left = if let Some(x) = &q_rc.borrow().left {
                            Some(Rc::clone(&x))
                        } else { None };
                        q_right = if let Some(x) = &q_rc.borrow().right {
                            Some(Rc::clone(&x))
                        } else { None };
                        p_left = if let Some(x) = &p_rc.borrow().left {
                            Some(Rc::clone(&x))
                        } else { None };
                        p_right = if let Some(x) = &p_rc.borrow().right {
                            Some(Rc::clone(&x))
                        } else { None };
                        return Solution::is_same_tree(
                            q_left, 
                            p_left) 
                            && 
                        Solution::is_same_tree(
                            q_right,
                            p_right
                        );
                    } else {
                        return false
                    }
                } else {
                    return false
                }
            },
            None => {
                if let Some(_) = q {
                    return false
                } else {
                    return true
                }
            }
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