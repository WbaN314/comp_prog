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
  pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {

      let mut fast = &mut head.clone();

      let mut dummy = Some(Box::new(ListNode{
        next: head,
        val: 0
      }));

      let mut slow = &mut dummy;

      for _ in 0..n {
          fast = &mut fast.as_mut().unwrap().next;
      }

      while fast.is_some() {
        fast = &mut fast.as_mut().unwrap().next;
        slow = &mut slow.as_mut().unwrap().next;
      }
      
      if slow.as_ref().is_some() && slow.as_ref().unwrap().next.as_ref().is_some() {
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
      } else {
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.take();
      };

      return dummy.unwrap().next
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