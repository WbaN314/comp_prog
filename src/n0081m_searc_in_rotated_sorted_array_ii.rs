// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        
        for i in nums {
            if i == target {
                return true
            }
        }

        false
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 0), true);
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 3), false);
     }
 }