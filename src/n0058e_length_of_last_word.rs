// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn length_of_last_word(mut s: String) -> i32 {
        s.split_ascii_whitespace()
            .last()
            .map(|s| s.len() as i32)
            .unwrap_or(0)
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