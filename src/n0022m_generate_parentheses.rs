// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

use std::collections::HashSet;
use std::collections::HashMap;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut cache: HashMap<i32, Vec<String>> = HashMap::new();
        Self::calc_parenthesis(n, &mut cache)
    }

    fn merge_into(left: Vec<String>, right: Vec<String>, set: &mut HashSet<String>) {
        for i in left.iter() {
            for j in right.iter() {
                set.insert(i.clone() + j);
                set.insert(j.clone() + i);
            }
        }
    }

    fn calc_parenthesis(n: i32, cache: &mut HashMap<i32, Vec<String>>) -> Vec<String> {
        match (cache.get(&n), n) {
            (Some(v), _) => return v.clone(),
            (None, 0) => return vec![],
            (None, 1) => return vec![String::from("()")],
            _ => {
                let mut set = HashSet::new();
                for i in 1..=n/2 {

                    let left = Self::calc_parenthesis(i, cache);
                    let right = Self::calc_parenthesis(n-i, cache);

                    Self::merge_into(left, right, &mut set);
                }

                for p in Self::calc_parenthesis(n - 1, cache) {
                    set.insert(String::from("(") + &p + ")");
                }
                
                let solution: Vec<String> = set.into_iter().collect();
                cache.insert(n, solution.clone());
                return solution
            }
        }
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        Solution::generate_parenthesis(3);
     }
 }