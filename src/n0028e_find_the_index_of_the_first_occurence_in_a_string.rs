// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(index) = haystack.find(&needle) {
            return index as i32
        } else {
            return -1
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