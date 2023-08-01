// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
       
        let mut word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut cache = vec![vec![-1; word2.len() + 1]; word1.len() + 1];

        for i in 0..=word1.len() {
            for j in 0..=word2.len() {
                
                if i == 0 {
                    cache[i][j] = j as i32;
                } else if j == 0 {
                    cache[i][j] = i as i32;
                } else if word1[i - 1] == word2[j - 1] {
                    cache[i][j] = cache[i - 1][j - 1];
                } else {
                    cache[i][j] = 1 + cache[i - 1][j - 1].min(cache[i - 1][j].min(cache[i][j - 1]));
                }
            }
        }
        return cache[word1.len()][word2.len()];
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