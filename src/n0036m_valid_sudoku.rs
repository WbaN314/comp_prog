// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

use std::collections::HashSet;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in &board {
            if !Self::check_row(&row) {
                return false
            }
        }
        for column in 0..board.len() {
            if !Self::check_column(&board, column) {
                return false
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                if !Self::check_box(&board, (i*3,j*3)) {
                    return false
                }
            }
        }
        true
    }

    fn check(set: &mut HashSet<char>, c: &char) -> bool {
        match c {
            '.' => (),
            c => {
                if !set.insert(*c) {
                    return false
                }
            }
        }
        true
    }

    fn check_row(row: &Vec<char>) -> bool {
        let mut set = HashSet::new();
        for c in row {
            if !Self::check(&mut set, c) {
                return false
            }
        };
        true
    }

    fn check_column(board: &Vec<Vec<char>>, c: usize) -> bool {
        let mut set = HashSet::new();
        for i in 0..board.len() {
            if !Self::check(&mut set, &board[i][c]) {
                return false
            }
        }
        true
    }

    fn check_box(board: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
        let mut set = HashSet::new();
        for i in 0..3 {
            for j in 0..3 {
                if !Self::check(&mut set, &board[start.0 + i][start.1 + j]) {
                    return false
                }
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
     }
 }