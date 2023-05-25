// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = matrix.len();
        let mut n = matrix[0].len();

        let mut solution: Vec<i32> = Vec::with_capacity(n * m);

        for i in 0..n {
            solution.push(matrix[0][i])
        }

        let mut i = 0;
        let mut j = n - 1;
        loop {

            if solution.len() >= matrix.len() * matrix[0].len() {break}
            // start top right m down
            for _ in 1..m {
                i += 1;
                solution.push(matrix[i][j]);
            }
            m = i32::max(0, m as i32 - 1) as usize;

            if solution.len() >= matrix.len() * matrix[0].len() {break}
            // go left
            for _ in 1..n {
                j -= 1;
                solution.push(matrix[i][j]);
            }
            n = i32::max(0, n as i32 - 1) as usize;

            if solution.len() >= matrix.len() * matrix[0].len() {break}
            // up again
            for _ in 1..m {
                i -= 1;
                solution.push(matrix[i][j]);
            }
            m = i32::max(0, m as i32 - 1) as usize;

            if solution.len() >= matrix.len() * matrix[0].len() {break}
            // and right
            for _ in 1..n {
                j += 1;
                solution.push(matrix[i][j]);
            }
            n = i32::max(0, n as i32 - 1) as usize;
            if solution.len() >= matrix.len() * matrix[0].len() {break}
        }

        println!("{solution:?}");
        solution
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::spiral_order(vec![vec![1,2,3,4], vec![5,6,7,8], vec![9,10,11,12]]), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
     }
 }