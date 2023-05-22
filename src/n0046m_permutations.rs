// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut solution: Vec<Vec<i32>> = Vec::new();
        Self::solve(&mut solution, nums, vec![]);
        solution
    }

    fn solve(solution: &mut Vec<Vec<i32>>, nums: Vec<i32>, current: Vec<i32>) {

        if nums.len() == 0 {
            solution.push(current);
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