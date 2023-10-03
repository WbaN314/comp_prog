// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

        if chars[0] == 0 {
            return 0
        }   

        // valid shows # of valid chars combinantions starting from index
        let mut valid: Vec<i32> = vec![0; chars.len() + 1];

        // all converted is 1
        valid[chars.len()] = 1;

        if chars[chars.len() - 1] != 0 {
            valid[chars.len() - 1] = 1;
        }

        for i in (0..chars.len() - 1).rev() {

            // starting with 0 - invalid index
            if chars[i] == 0 {
                valid[i] = 0;
                continue;
            }

            if chars[i] == 1 || (chars[i] == 2 && chars[i + 1] <= 6) {
                valid[i] = valid[i+1] + valid[i+2]
            } else {
                valid[i] = valid[i+1]
            } 
        }
        valid[0]
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::num_decodings(String::from("2101")), 1);
     }
 }