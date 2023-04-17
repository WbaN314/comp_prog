// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

        if (nums.len() == 0) {
            return 0
        }

        let mut last = nums[0];
        let mut slow_idx = 1;
        for i in 1..nums.len() {
            if (last == nums[i]) {
                continue
            }
            nums[slow_idx] = nums[i];
            slow_idx += 1;
            last = nums[i];
        }

        slow_idx as i32
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