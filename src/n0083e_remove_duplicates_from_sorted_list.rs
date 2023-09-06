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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        if head.is_none() {
          return None;
        }

        let mut p1 = head.as_mut().unwrap();

        while let Some(p2) = p1.next.as_mut() {
            if p1.val == p2.val {
                p1.next = p2.next.take();
            } else {
                p1 = p1.next.as_mut().unwrap();
            }
        }

        head
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