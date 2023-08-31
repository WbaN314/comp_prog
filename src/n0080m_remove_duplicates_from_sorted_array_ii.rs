// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last = 0;
        let mut scout = 1;
        let mut seen = 1;

        while scout < nums.len() {

            if nums[scout] == nums[last] {
                seen += 1
            } else {
                seen = 1
            }

            if seen <= 2 {
                last += 1;
                nums[last] = nums[scout];
            }

            scout += 1
        }

        last as i32 + 1
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