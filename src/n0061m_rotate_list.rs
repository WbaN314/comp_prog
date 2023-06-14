// https://leetcode.com/problems
pub struct Solution {}

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {

        if head.is_none() {
          return head;
        }
        
        let mut helper: Vec<*mut ListNode> = Vec::new();
        
        let mut ptr = head.as_mut();
        while let Some(node) = ptr {
          helper.push(&mut (**node));
          ptr = node.next.as_mut();
        }
        
        let len = helper.len();
        let k = k as usize % len;

        if k == 0 {
          return head
        }

        // println!("{:?}", helper);

        unsafe {
          (**helper.last_mut().unwrap()).next = head;
          (*helper[len - k as usize - 1]).next.take()
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