// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n)
        .fold((1,0), |(cur, last), _| (cur + last, cur))
        .0 
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