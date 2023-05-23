// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut solution: HashMap<[u8;26], Vec<String>> = HashMap::new();

        let mut key: [u8;26];
        for s in strs {
            key = [0;26];
            for c in s.bytes() {
                key[(c - b'a') as usize] += 1;
            }
            solution.entry(key).and_modify(|vec| vec.push(s.clone())).or_insert(vec![s]);
        }
        solution.into_values().collect()
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