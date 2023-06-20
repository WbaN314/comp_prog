// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_i = u128::from_str_radix(&a, 2).unwrap();
        let b_i = u128::from_str_radix(&b, 2).unwrap();
        format!("{:b}", a_i+b_i)
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