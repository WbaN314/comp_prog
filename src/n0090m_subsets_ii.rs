// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

// 1 2 2 3
// [] [1] [1 2] [1 2 2] [1 2 2 3] [1 2 3] [1 2]!! [1 2 3]!! [1 3] [2] [2 2] [2 2 3] [2 3] [2]!! [2 3]!! [3]

use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut solution = Vec::new();
        let candidate = Vec::with_capacity(nums.len());
        solution.push(candidate.clone());
        Self::helper(&mut solution, &candidate, &nums, 0);    
        solution
    }

    fn helper(solution: &mut Vec<Vec<i32>>, candidate: &Vec<i32>, nums: &Vec<i32>, start: usize) {
        let mut last: Option<i32> = None;
        for i in start..nums.len() {
            if let Some(last) = last {
                if last == nums[i] {
                    continue
                }
            }
            last = Some(nums[i]);
            let mut new_candidate = candidate.clone();
            new_candidate.push(nums[i]);
            Self::helper(solution, &new_candidate, nums, i + 1);
            solution.push(new_candidate);
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