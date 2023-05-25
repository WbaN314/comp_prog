// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {

        let mut current_sum = 0;
        let mut best_sum = i32::MIN;
        
        for x in nums {
            current_sum = i32::max(x, current_sum + x);
            best_sum = i32::max(best_sum, current_sum);
        }

        best_sum
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