// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {

    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for b in s.chars() {
            stack.push(match b {
                '{' => '}',
                '[' => ']',
                '(' => ')',
                _ => {
                    if Some(b) == stack.pop() {
                        continue
                    } else {
                        return false
                    }
                }
            })
        }

        stack.is_empty()
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