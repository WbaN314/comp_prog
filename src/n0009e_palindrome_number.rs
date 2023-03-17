// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }
        let mut x_mut = x;
        let mut last_digit: i32;
        let mut reversed: i32 = 0;

        while x_mut > 0 {
            last_digit = x_mut % 10;
            reversed = reversed * 10 + last_digit;
            x_mut = x_mut / 10;
        }

        if reversed == x {
            return true
        }

        false
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