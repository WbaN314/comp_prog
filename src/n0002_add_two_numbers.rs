// https://leetcode.com/problems/two-sum/
pub struct Solution {}

// START SUBMISSION CODE

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

type BoxNode = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::add(l1, l2, 0, ListNode::new(0))
    }
    
    

    fn add(mut l1: BoxNode, mut l2: BoxNode, mut overflow: i32, mut sol: ListNode) -> BoxNode {
        
        if l1.is_none() && l2.is_none() && overflow == 0 {
            return None;
        }
        
        if let Some(n1) = l1 {
            overflow += n1.val;
            l1 = n1.next;
        }
        
        if let Some(n2) = l2 {
            overflow += n2.val;
            l2 = n2.next;
        }
        
        sol.val = if overflow > 9 { overflow - 10 } else { overflow };
        sol.next = Solution::add(l1, l2, overflow / 10, ListNode::new(0));
        Some(Box::new(sol))
    }
}

 
 // END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(1, 1);
     }
 }