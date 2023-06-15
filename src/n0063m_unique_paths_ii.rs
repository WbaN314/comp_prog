// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {

        // handling the unfair testcase
        if obstacle_grid[0][0] == 1 {
            return 0
        }

        let mut table: Vec<Vec<i32>> = Vec::new();
        for _ in 0..obstacle_grid.len() {
            table.push(vec![0; obstacle_grid[0].len()])
        }
        
        table[0][0] = 1;

        // could be done even better by only using the last row as a cache, not whole table required
        // but this does the trick
        for i in 0..table.len() {
            for j in 0..table[0].len() {
                table[i][j] += {
                    if obstacle_grid[i][j] == 1 {
                        0
                    } else {
                        match (i > 0, j > 0) {
                            (true, true) => table[i-1][j] + table[i][j-1],
                            (true, false) => table[i-1][j],
                            (false, true) => table [i][j-1],
                            (false, false) => 0
                        }
                    }
                }
            }
        }
        *table.last().unwrap().last().unwrap()
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