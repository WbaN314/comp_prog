// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {

        let is_negative = (dividend < 0) ^ (divisor < 0);

        if dividend > 0 {dividend = -dividend};
        if divisor > 0 {divisor = -divisor};

        let mut result = 0;

        while dividend <= divisor {
            result -= 1;
            dividend -= divisor;
        }

        if is_negative {
            result
        } else if result == i32::MIN {
            i32::MAX
        } else {
            -result
        }
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::divide(10, 3), 3)
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::divide(7, -3), -2)
     }
     #[test]
     fn test_3() {
        assert_eq!(Solution::divide(1, 1), 1)
     }
     #[test]
     fn test_4() {
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647)
     }
     #[test]
     fn test_5() {
        assert_eq!(Solution::divide(-2147483648, 1), -2147483648)
     }
 }