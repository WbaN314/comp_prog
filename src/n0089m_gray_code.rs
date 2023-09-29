// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        if n == 0 {
            return Vec::new()
        } else if n == 1 {
            return vec![0,1]
        } else {
            let front = Self::gray_code(n-1);
            let back = front.clone();
            let bit_flip: i32 = 1<<(n-1);

            return front.into_iter().chain(back.into_iter().rev().map(|x| x | bit_flip)).collect()
        };
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