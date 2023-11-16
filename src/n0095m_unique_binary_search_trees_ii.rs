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

  pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
      if n == 0 {
        Vec::new()
      } else {
        Self::generate(1, n)
      }
  }

  fn generate(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {

    if start > end {
      return vec![None];
    }

    let mut all_trees = Vec::new();

    for i in start..=end {
      let left_trees = Self::generate(start, i-1);
      let right_trees = Self::generate(i+1, end);
      
      for left in &left_trees {
        for right in &right_trees {
          let new_head = Rc::new(RefCell::new(TreeNode::new(i)));
          new_head.borrow_mut().left = left.clone();
          new_head.borrow_mut().right = right.clone();
          all_trees.push(Some(new_head));
        }
      }
    }

    return all_trees
  
  }
    
}
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
      assert_eq!(Solution::generate_trees(0).len(), 0);
      assert_eq!(Solution::generate_trees(1).len(), 1);
      assert_eq!(Solution::generate_trees(2).len(), 2);
      assert_eq!(Solution::generate_trees(3).len(), 5);
     }
 }