// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = Vec::new();

        let up: usize = (n * n) as usize;
        let mut m: usize = n as usize;
        let mut n: usize = n as usize;

        for _ in 0..n {
            matrix.push(vec![0; n as usize]);
        }

        let mut current: i32 = 1;

        for i in 0..n {
            matrix[0][i as usize] = current;
            current += 1;
        }

        let mut i = 0;
        let mut j: usize = n - 1;

        loop {

            if current as usize > up {break}
            // start top right m down
            for _ in 1..m {
                i += 1;
                matrix[i][j] = current;
                current += 1;
            }
            m = i32::max(0, m as i32 - 1) as usize;

            if current as usize > up {break}
            // go left
            for _ in 1..n {
                j -= 1;
                matrix[i][j] = current;
                current += 1;
            }
            n = i32::max(0, n as i32 - 1) as usize;

            if current as usize > up {break}
            // up again
            for _ in 1..m {
                i -= 1;
                matrix[i][j] = current;
                current += 1;
            }
            m = i32::max(0, m as i32 - 1) as usize;

            if current  as usize > up {break}
            // and right
            for _ in 1..n {
                j += 1;
                matrix[i][j] = current;
                current += 1;
            }
            n = i32::max(0, n as i32 - 1) as usize;
            if current as usize > up {break}
        }

        matrix
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