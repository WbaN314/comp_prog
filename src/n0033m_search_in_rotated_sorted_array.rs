use std::ops::Index;

// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let split = nums.partition_point(|&x| x >= nums[0]);
        if target < nums[0] {
            if let Ok(i) = nums[split..].binary_search(&target) {
                (i + split) as i32
            } else {
                -1
            }
        } else {
            if let Ok(i) = nums[..split].binary_search(&target) {
                i as i32
            } else {
                -1
            }
        }
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let a = vec![4,5,6,7,0,1,2];
        assert_eq!(Solution::search(a, 0), 4);
     }
     #[test]
     fn test_2() {
        let a = vec![1];
        assert_eq!(Solution::search(a, 1), 0);
     }
 }