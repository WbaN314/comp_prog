// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = nums.partition_point(|&x| x < target);
        if start == nums.len() || nums[start] != target {
            return vec![-1, -1]
        }
        let end = nums[start..].partition_point(|&x| x <= target) + start;
        vec![start as i32, end as i32 - 1]
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let s = vec![5,7,7,8,8,10];
        assert_eq!(Solution::search_range(s, 8), vec![3,4]);
     }
 }