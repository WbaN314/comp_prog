// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut solution: Vec<Vec<i32>> = vec![];
        nums.sort_unstable();
        let set: HashSet<i32> = HashSet::from_iter(nums.clone());

        for i in 0..nums.len()-2 {
            for j in i+1..nums.len() - 1 {
                let req = -(nums[i] + nums[j]);
                if set.contains(&req) {
                    if req < nums[j] {
                        break
                    } else if req == nums[j] && req != nums[j+1] {
                        break
                    }
                    solution.push(vec![nums[i], nums[j], req])
                }
            }
        }
        HashSet::<Vec<i32>>::from_iter(solution).into_iter().collect()
    }
}
 
 // END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let inp = vec![0, 1, 1];
        let out: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(inp), out);
     }

     fn test_2() {
        let inp = vec![0, 0, 0];
        let out = vec![vec![0, 0, 0]];
        assert_eq!(Solution::three_sum(inp), out);
     }

     fn test_3() {
        let inp = vec![-1,0,1,2,-1,-4];
        let out = vec![vec![-1,-1,2], vec![-1,0,1]];
        assert_eq!(Solution::three_sum(inp), out);
     }
 }