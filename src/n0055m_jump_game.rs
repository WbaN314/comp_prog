// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reachable_index: usize = 0;
        for i in 0..nums.len() {
            if i > max_reachable_index {
                return false;
            } else {
                max_reachable_index = std::cmp::max(max_reachable_index, i + nums[i] as usize)
            }
        }
        return true;
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::can_jump(vec![2,3,1,1,4]), true);
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::can_jump(vec![1,1,2,2,0,1,1]), true);
     }
     #[test]
     fn test_3() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0]), true);
     }
     #[test]
     fn test_4() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 1]), false);
     }
 }