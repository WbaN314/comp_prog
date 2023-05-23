// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        x.powi(n)
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