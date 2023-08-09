// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // concatenate all rows into a single vector
        let mut v = Vec::with_capacity(matrix.len() * matrix[0].len());
        for row in matrix {
            v.extend(row);
        }
        v.binary_search(&target).is_ok()
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