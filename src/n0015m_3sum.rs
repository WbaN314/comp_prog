// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {

        let mut solution: Vec<Vec<i32>> = vec![];
        if nums.len() == 0 {
            return solution
        }

        nums.sort_unstable();

        for l in 0..nums.len()-2 {
            if l > 0 && nums[l] == nums[l - 1] {continue};
            let mut m = l+1;
            let mut r = nums.len() - 1;
            while m < r {
                let val = nums[l] + nums[m] + nums[r];
                match (val > 0, val < 0) {
                    (true, _) => r -= 1,
                    (_, true) => m += 1,
                    _ => {
                        solution.push(vec![nums[l], nums[m], nums[r]]);
                        m += 1;
                        while nums[m] == nums[m - 1] && m < r {
                            m += 1;
                        }
                    }
                }
            }
        };

        solution
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