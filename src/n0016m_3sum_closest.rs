// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        
        nums.sort_unstable();

        let mut solution = nums[0] + nums[1] + nums[2];
        let mut best_dist = (nums[0] + nums[1] + nums[2] - target).abs();

        for l in 0..nums.len() - 2 {
            let mut m = l + 1;
            let mut r = nums.len() - 1;
            while m < r {
                let s = nums[l] + nums[m] + nums[r];
                let dist = target - s;
                if dist.abs() <  best_dist {
                    best_dist = dist.abs();
                    solution = s;
                };
                match (dist < 0, dist > 0) {
                    (true, _) => {
                        r -= 1
                    },
                    (_, true) => {
                        m += 1
                    },
                    _ => return solution
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
        let inp1 = vec![-1,2,1,-4];
        let inp2 = 1;
        assert_eq!(Solution::three_sum_closest(inp1, inp2), 2);
     }

     #[test]
     fn test_2() {
        let inp1 = vec![0, 1, 2];
        let inp2 = 0;
        assert_eq!(Solution::three_sum_closest(inp1, inp2), 3);
     }

     #[test]
     fn test_3() {
        let inp1 = vec![4,0,5,-5,3,3,0,-4,-5];
        let inp2 = -2;
        assert_eq!(Solution::three_sum_closest(inp1, inp2), -2);
     }

     #[test]
     fn test_4() {
        let inp1 = vec![4,5,-5,3,0,-4];
        let inp2 = -2;
        assert_eq!(Solution::three_sum_closest(inp1, inp2), -2);
     }
 }