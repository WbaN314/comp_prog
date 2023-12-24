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
  pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
      let mut vec = Vec::new();

      Self::in_order(root, &mut vec);

      let mut first = None;
      let mut second= None;
      for i in 0..vec.len() {
        match first {
          None => {
            if unsafe {*vec[i] > *vec[i+1]} {
              first = Some(vec[i]);
              second = Some(vec[i]);
            }
          },
          Some(_) => {
            if unsafe {*vec[i] < *second.unwrap()} {
              second = Some(vec[i]);
            }
          }
        }
      }

      unsafe {std::ptr::swap(first.unwrap(), second.unwrap())};
  }

  pub fn in_order(root: &mut Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<*mut i32>) {
    if let Some(node) = root {
      let borrow = node.as_ptr();
      unsafe {
        Self::in_order(&mut (*borrow).left, vec);
        vec.push(&mut (*borrow).val);
        Self::in_order(&mut (*borrow).right, vec);
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