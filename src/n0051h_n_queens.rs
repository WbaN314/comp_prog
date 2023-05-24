// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut positions: Vec<u8> = Vec::with_capacity(n as usize);
        let mut solutions: Vec<Vec<u8>> = Vec::new();
        Self::backtracking(&mut solutions, &mut positions, n);
        Self::convert_solutions(solutions, n)
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

    fn convert_solutions(solutions: Vec<Vec<u8>>, n: i32) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        
        for solution in solutions {
            let mut tmp: Vec<String> = Vec::new();
            for column in solution {
                let mut row = vec!['.'; n as usize];
                row[column as usize] = 'Q';
                tmp.push(row.into_iter().collect())
            }
            result.push(tmp);
        }

        result
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"]])
     }
 }