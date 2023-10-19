// https://leetcode.com/problems
pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// START SUBMISSION CODE

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));

        let mut reverse_after = &mut dummy_head;

        for _ in 0..left-1 {
          reverse_after = &mut reverse_after.as_mut().unwrap().next;
        }
        // node_ref points to the node before the first node to be reversed

        // desired iterations:
        // reverse_after -> start -> next -> next2 -> next3 -> ..
        // reverse_after -> next -> start -> next2 -> next3 -> ..
        // reverse_after -> next2 -> next -> start -> next3 -> ..

        // start points to the first node to be reversed
        let mut start = reverse_after.as_mut().unwrap().next.take();
        // reverse_after -> None, start -> next -> next2 -> next3 -> ..

        for _ in left..right {
          let mut next = start.as_mut().unwrap().next.take();
          // reverse_after -> x, start -> None, next -> next2 -> next3 -> ..
          start.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
          // reverse_after -> x, start -> next2 -> next3 -> .., next -> None
          next.as_mut().unwrap().next = reverse_after.as_mut().unwrap().next.take();
          // reverse_after -> None, next -> x, start -> next2 -> next3 -> ..
          reverse_after.as_mut().unwrap().next = next;
          // reverse_after -> next -> x, start -> next2 -> next3 -> ..
        }

        for _ in left..right {
          reverse_after = &mut reverse_after.as_mut().unwrap().next;
        }
        reverse_after.as_mut().unwrap().next = start;

        return dummy_head.unwrap().next;
    }
}

// END SUBMISSION CODE

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = Some(Box::new(ListNode::new(1)));
        Solution::reverse_between(head, 1, 1);
    }
}
