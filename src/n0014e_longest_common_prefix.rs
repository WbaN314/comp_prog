// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: Vec<char> = strs[0].chars().collect();

        for i in 1..strs.len() {
            let new_chars: Vec<char> = strs[i].chars().collect();
            if prefix.len() > new_chars.len() {
                prefix = prefix[0..new_chars.len()].to_vec();
            }
            for j in 0..new_chars.len() {
                if j >= prefix.len() {
                    break;
                };
                if prefix[j] != new_chars[j] {
                    prefix = prefix[0..j].to_vec();
                }
            };
        };
        return prefix.into_iter().collect()
    }
}
 
 // END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
        let out = String::from("fl");
        assert_eq!(Solution::longest_common_prefix(strs), out);
     }
 }