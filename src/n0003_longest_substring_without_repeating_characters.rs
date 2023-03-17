// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {

        let char_vec: Vec<char> = s.chars().collect();

        let mut used_chars: HashSet<char> = HashSet::new();
        let mut max_length = 0;
        let mut left: usize = 0;
        let mut right: usize = 0;

        while right < char_vec.len() {
            
            while used_chars.contains(&char_vec[right]) {
                used_chars.remove(&char_vec[left]);
                left += 1;
            }

            used_chars.insert(char_vec[right]);

            if right - left + 1 > max_length {
                max_length = right - left + 1
            }

            right += 1;
        }

        return max_length as i32
 
    }
}
 
 // END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
         assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
         assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
         assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
     }
 }