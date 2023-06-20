// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {

        let mut cache = vec![0; grid[0].len()];

        cache[0] = grid[0][0];
        for i in 1..cache.len() {
            cache[i] = cache[i-1] + grid[0][i];
        }

        for i in 1..grid.len() {
            cache[0] += grid[i][0];
            for j in 1..cache.len() {
                cache[j] = i32::min(cache[j-1], cache[j]) + grid[i][j];
            }
        }

        *cache.last().unwrap()
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