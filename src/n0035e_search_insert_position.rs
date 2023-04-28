// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
               nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2)
     }
 }