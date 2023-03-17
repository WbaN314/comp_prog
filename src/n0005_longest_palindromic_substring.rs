// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        let str_vec: Vec<char> = s.chars().collect();

        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut size: usize = right - left;

        for i in 0..str_vec.len() {
            let (left_candidate, right_candidate) = Self::build_palindrome(&str_vec, i);
            if right_candidate - left_candidate > size {
                left = left_candidate;
                right = right_candidate;
                size = right_candidate - left_candidate;
            }
        }

        return str_vec[left..=right].iter().collect()
    }

    fn build_palindrome(str_vec: &Vec<char>, root: usize) -> (usize, usize) {
        // odd
        let mut offset = 1;
        let odd = loop {
            if offset > root || root + offset >= str_vec.len() {
                break (root - offset + 1, root + offset - 1)
            } else if str_vec[root - offset] == str_vec[root + offset]{
                offset += 1;
                continue
            } else {
                break (root - offset + 1, root + offset -1)
            }
        };
        
        //even
        offset = 0;
        let even = loop {
            if offset > root || root + offset + 1 >= str_vec.len() {
                if offset == 0 {
                    break (root, root)
                } else {
                    break (root - offset + 1, root + offset)
                }
            } else if str_vec[root - offset] == str_vec[root + offset + 1] {
                offset += 1;
                continue
            } else {
                if offset == 0 {
                    break (root, root)
                } else {
                    break (root - offset + 1, root + offset)
                }
            }
        };
        return if even.1 - even.0 > odd.1 - odd.0 {even} else {odd}
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