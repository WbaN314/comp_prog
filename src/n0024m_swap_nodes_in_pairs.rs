// https://leetcode.com/problems
pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// START SUBMISSION CODE

impl Solution {
  pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      match head {
        Some(mut node1) => {
          match node1.next {
            Some(mut node2) => {
              node1.next = Self::swap_pairs(node2.next);
              node2.next = Some(node1);
              Some(node2)
            },
            None => Some(node1)
          }
        },
        None => head
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