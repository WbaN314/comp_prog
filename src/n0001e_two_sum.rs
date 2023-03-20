// https://leetcode.com/problems/two-sum/
pub struct Solution {}

// START SUBMISSION CODE

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut m: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match m.get(&(target - *v)) {
                Some(&i2) => return vec![i as i32, i2],
                None => m.insert(*v, i as i32),
            };
        }
        vec![]
    }
}
 
 // END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
         assert_eq!(vec![1, 0], Solution::two_sum(vec![2, 7, 11, 15], 9));
         assert_eq!(vec![2, 1], Solution::two_sum(vec![3, 2, 4], 6));
     }
 }