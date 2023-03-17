// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let char_vec = s.as_bytes();

        let mut i = 0;

        while let Some(c) = char_vec.get(i) {
            if c != &b' ' {break}
            i += 1;
        }

        let mut positive: i64 = 1;
        match char_vec.get(i) {
            Some(c) if c == &b'+' => {
                i += 1;
            }
            Some(c) if c == &b'-' => {
                positive = -1;
                i += 1
            }
            _ => ()
        }

        let mut solution: i64 = 0;
        while let Some(c) = char_vec.get(i) {
            if let Some(n) = (*c as char).to_digit(10) {
                solution = solution * 10 + n as i64;

                if positive * solution > i32::MAX as i64 {
                    return i32::MAX as i32;
                } else if positive * solution < i32::MIN as i64 {
                    return i32::MIN as i32;
                }

                i += 1;
            } else {
                break
            }
        }

        return (positive * solution) as i32

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