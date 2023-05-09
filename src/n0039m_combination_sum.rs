// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut current = Vec::new();
        let mut solution = Vec::new();
        Self::helper(&candidates, target, &mut solution, &mut current, 0, 0);
        solution
    }

    fn helper(candidates: &Vec<i32>, target: i32, solution: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, sum: i32, idx: usize) {
        match sum {
            x if x > target => return,
            x if x == target => {
                solution.push(current.clone());
                return
            }
            _ => {
                for i in idx..candidates.len() {
                    current.push(candidates[i]);
                    Self::helper(candidates, target, solution, current, sum + candidates[i], i);
                    current.pop();
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