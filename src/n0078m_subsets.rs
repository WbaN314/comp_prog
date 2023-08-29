// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut solution = Vec::new();
        let candidate = Vec::with_capacity(nums.len());
        solution.push(candidate.clone());
        Self::helper(&mut solution, &candidate, &nums, 0);    
        solution
    }

    fn helper(solution: &mut Vec<Vec<i32>>, candidate: &Vec<i32>, nums: &Vec<i32>, start: usize) {
        for i in start..nums.len() {
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