// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut next = 0;
        for j in 0..=1 {
            for i in next..nums.len() {
                if nums[i] == j {
                    nums.swap(i, next);
                    next += 1;
                }
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
     }
 }