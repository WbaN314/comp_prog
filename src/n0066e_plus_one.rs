// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut new = digits.clone();
        let mut prepend = true;
        for i in (0..new.len()).rev() {
            match new[i] {
                9 => new[i] = 0,
                _ => {
                    new[i] += 1;
                    prepend = false;
                    break; 
                }
            }
        }
        if (prepend) {
            new.insert(0,1);
        }
        new
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