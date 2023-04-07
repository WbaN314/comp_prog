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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut values = Vec::new();

        for mut list in lists {
            while let Some(node) = list {
                values.push(node.val);
                list = node.next;
            }
        };

        values.sort_unstable();

        let mut start = ListNode::new(0);
        let mut reference = &mut start;
        for v in values {
            reference.next = Some(Box::new(ListNode::new(v)));
            reference = reference.next.as_mut().unwrap();
        };
         
        start.next
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