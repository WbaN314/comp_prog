// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        
        k = k - 1;
        let mut solution: Vec<i32> = Vec::new();
        let mut options: Vec<i32> = (1..=n).collect();
        let mut options_per_skip: usize = (1..options.len()).product();
        let mut skip = k as usize / options_per_skip;
        
        while options.len() > 1 {
            solution.push(options.remove(skip));

            k %= options_per_skip as i32; 
            options_per_skip /= options.len();
            skip = k as usize / options_per_skip;
        }

        solution.append(&mut options);

        solution.into_iter().map(|i| i.to_string()).collect()
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::get_permutation(3, 1), String::from("123"));
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::get_permutation(2, 2), String::from("21"));
     }
     #[test]
     fn test_3() {
        assert_eq!(Solution::get_permutation(3, 2), String::from("132"));
     }
     #[test]
     fn test_4() {
        assert_eq!(Solution::get_permutation(3, 4), String::from("231"));
     }
 }