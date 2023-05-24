// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut positions: Vec<u8> = Vec::with_capacity(n as usize);
        let mut solutions: Vec<Vec<u8>> = Vec::new();
        Self::backtracking(&mut solutions, &mut positions, n);
        solutions.len() as i32
    }

    fn backtracking(solutions: &mut Vec<Vec<u8>>, positions: &mut Vec<u8>, n: i32) {
        
        if positions.len() == n as usize {
            solutions.push(positions.clone());
            return
        }

        for i in 0..n {
            if Self::valid_position(positions, i as u8) {
                positions.push(i as u8);
                Self::backtracking(solutions, positions, n);
                positions.pop();
            }
        }
    }

    fn valid_position(positions: &mut Vec<u8>, y: u8) -> bool {
        for i in (0..positions.len()).rev() {
            // directly above
            if positions[i] == y {
                return false
            }
            // above left
            if positions[i] + positions.len() as u8 == y + i as u8 {
                return false
            }
            // above right
            if positions[i] + i as u8 == y + positions.len() as u8 {
                return false
            }
        }
        true
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::total_n_queens(1), 1)
     }
 }