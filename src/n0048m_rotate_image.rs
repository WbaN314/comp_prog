// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let l = matrix.len();
        
        // transpose
        
        let mut tmp;
        for i in 0..l {
            for j in i..l {
                tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        
        // reverse
        
        for i in 0..l {
            for j in 0..l / 2 {
                tmp = matrix[i][j];
                matrix[i][j] = matrix[i][l - 1 - j];
                matrix[i][l - 1 - j] = tmp;
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
        let mut x = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        Solution::rotate(&mut x);
        assert_eq!(x, vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]]);
     }
 }