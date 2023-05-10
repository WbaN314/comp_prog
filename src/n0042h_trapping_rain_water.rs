// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        
        let mut stack: Vec<usize> = Vec::new();
        let mut solution = 0;
        let mut n: i32;

        for i in 1..height.len() {
            n = height[i-1] - height[i];

            if n > 0 {
                for _ in 0..n {
                    stack.push(i);
                }
            } else if n < 0 {
                for _ in 0..-n {
                    if let Some(v) = stack.pop() {
                        solution += i - v;
                    }
                }
            }
        }
        solution as i32
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
     }
 }