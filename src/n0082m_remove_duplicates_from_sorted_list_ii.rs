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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

       fn recurse(node: Option<Box<ListNode>>, prev: Option<i32>) -> Option<Box<ListNode>> {
            if let Some(mut n) = node {
                if Some(n.val) == prev {
                    recurse(n.next, Some(n.val))
                } else {
                    match n.next {
                        Some(n2) if n2.val == n.val => recurse(Some(n2), Some(n.val)),
                        _ => {
                            n.next = recurse(n.next, Some(n.val));
                            Some(n)
                        }
                    }
                }
            } else {
                None
            }
       }
       recurse(head, None)
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