// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve_sudoku_helper(board, 0);
    }

    fn solve_sudoku_helper(board: &mut Vec<Vec<char>>, n: usize) -> bool {

        if n > 80 {
            return true
        }

        let (i, j) = (n % 9, n / 9);

        match board[i][j] {
            '1'..='9' => Self::solve_sudoku_helper(board, n + 1),
            '.' => {
                for c in '1'..='9' {
                    if Self::check_value(board, c, (i, j)) {
                        board[i][j] = c;
                        if Self::solve_sudoku_helper(board, n + 1) {
                            return true
                        } 
                        board[i][j] = '.';
                    }
                }
                return false
            }
            _ => unreachable!()
        }
    }

    fn check_value(board: &mut Vec<Vec<char>>, c: char, (i, j): (usize, usize)) -> bool {

        for k in 0..9 {

            if board[i][k] == c {
                return false
            }

            if board[k][j] == c {
                return false 
            }
            
            if board[i/3 * 3 + k % 3][j/3 * 3 + k / 3] == c {
                return false
            }
        };

        true
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