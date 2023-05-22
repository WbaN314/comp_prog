// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE
// runtime not optimal but easy adoption of 0046
use std::collections::HashSet;
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut solution: HashSet<Vec<i32>> = HashSet::new();
        Self::solve(&mut solution, nums, vec![]);
        solution.into_iter().collect()
    }

    fn solve(solution: &mut HashSet<Vec<i32>>, nums: Vec<i32>, current: Vec<i32>) {

        if nums.len() == 0 {
            solution.insert(current);
            return
        }

        for i in 0..nums.len() {
            let mut new_nums = nums.clone();
            let new = new_nums.remove(i);
            let mut new_v = current.clone();
            new_v.push(new);
            Self::solve(solution, new_nums , new_v);
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