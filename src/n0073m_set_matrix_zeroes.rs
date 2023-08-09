// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row0 = vec![false; matrix[0].len()];
        let mut col0 = vec![false; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    row0[j] = true;
                    col0[i] = true;
                }
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if row0[j] || col0[i] {
                    matrix[i][j] = 0;
                }
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
     }
 }