// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {

        let mut position = 0;
        let mut jump_width;
        let mut best;
        let mut best_i;
        let mut solution = 0;

        if nums.len() == 0 {
            return solution
        }

        while position < nums.len() - 1 {
            solution += 1;
            jump_width = nums[position];

            if position + jump_width as usize >= nums.len() - 1 {
                return solution
            }

            best = 0;
            best_i = 0;
            for i in 1..=jump_width {
                if nums[position + i as usize] - (jump_width - i) > best {
                    best = nums[position + i as usize] - (jump_width - i);
                    best_i = i;
                }
            }
            position = position + best_i as usize;
        }

        solution
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2)
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::jump(vec![2,3,0,1,4]), 2)
     }
     #[test]
     fn test_3() {
        assert_eq!(Solution::jump(vec![2,3,1]), 1)
     }
 }