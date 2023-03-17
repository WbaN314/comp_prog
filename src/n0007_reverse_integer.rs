// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut digits = Vec::new();
        let (mut x, positive) = if x > 0 {(x, true)} else {(-x, false)};

        if x == 0 {return 0}
        if x == -2147483648 {return 0}

        while x > 0 {
            digits.push(x % 10);
            x /= 10;
        }

        if digits[0] > 2 && digits.len() == 10 {
            return 0
        }

        let mut potential_solution: i32 = 0;
        for i in digits {
            potential_solution = potential_solution * 10 + i;
        }

        if positive && potential_solution < 0 || !positive && potential_solution < 0 {
            return 0
        } else {
            return if positive {potential_solution} else {-1 * potential_solution}
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