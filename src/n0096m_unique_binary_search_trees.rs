// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

use std::collections::HashMap;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // 0: 0
        // 1: 1
        // 2: 2
        // 3: 5
        // 4:

        // Let B(n) be the number of distinct binary search trees with n nodes
        // B(n) = B(0) * B(n-1) + B(1) * B(n - 2) + ... + B(n-1) * B(0)

        let mut mem = HashMap::new();

        match n {
            0 => 1,
            _ => {
                let mut num = 0;
                for i in 0..n {
                        num += Self::memorize_num_trees(i, &mut mem) * Self::memorize_num_trees(n-1 - i, &mut mem)
                }
                num
            }
        }
    }

    fn memorize_num_trees(n: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        if let Some(val) = mem.get(&n) {
            *val
        } else {
            let val = Self::num_trees(n);
            mem.insert(n, val);
            val
        }
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