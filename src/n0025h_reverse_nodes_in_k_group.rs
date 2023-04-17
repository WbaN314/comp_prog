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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {

      // mutable reference to node, used for end of k-block
      let mut node = &mut head;

      // move node k listnodes forward, if not possible return list as is
      for _ in 0..k {
        if let Some(n) = node {
          node = &mut n.next;
        } else {
          return head
        }
      }

      // Recurse on k+1th node and move value out of it leaving none in current scope
      let mut solution = Self::reverse_k_group(node.take(), k);

      // Build list backwards until the created none is reached
      while let Some(n) = head.take() {
        solution = Some(Box::new(ListNode { 
          val: n.val, 
          next: solution 
        }));
        head = n.next;
      }

      solution
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