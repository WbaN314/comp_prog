// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut solution: Vec<Vec<i32>> = Vec::new();

        if nums.len() < 4 {
            return solution
        }

        let mut ml;
        let mut mr;

        for l in 0..nums.len() - 3 {
            if l > 0 && nums[l] == nums[l - 1] {continue};
            for r in (l+3..nums.len()).rev() {
                if r + 1 < nums.len() && nums[r] == nums[r + 1] {continue};
                ml = l + 1;
                mr = r - 1;
                loop {
                    if ml >= mr {
                        break;
                    }
                    let curr = nums[l] as i64 + nums[ml] as i64 + nums[mr] as i64 + nums[r] as i64 - target as i64;
                    match (curr < 0, curr > 0) {
                        (true, false) => ml += 1,
                        (false, true) => mr -= 1,
                        (false, false) => {
                            solution.push(vec![nums[l], nums[ml], nums[mr], nums[r]]);
                            ml += 1;
                            while nums[ml] == nums[ml - 1] && ml < mr {
                                ml += 1;
                            }
                        },
                        _ => panic!("lol how did i get here")
                    }
                }
            }
        }
        return solution
    }
}

// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let nums = vec![2,2,2,2,2];
        let target = 8;
        assert_eq!(Solution::four_sum(nums, target), vec![vec![2,2,2,2]])
     }
 }