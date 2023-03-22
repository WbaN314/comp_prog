// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut last_value = 0;
        s.chars().fold(0, |acc, x| {
            let x_value = match x {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!()
            };
            let mut tmp = acc;
            if last_value < x_value {
                tmp += x_value - 2 * last_value;
            } else {
                tmp += x_value;
            }
            last_value = x_value;
            return tmp
        }
        )
    }
}
 
 // END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994)
     }
 }