// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut solution = vec![];
        Self::solve(&mut solution, String::new(), 0, 0, n as usize);
        solution
    }

    fn solve(solution: &mut Vec<String>, cur: String, open: usize, closed: usize, max: usize) {
        
        if cur.len() == 2 * max {
            solution.push(cur);
            return
        }

        if open < max {
            Self::solve(solution, cur.clone() + "(", open + 1, closed, max);
        }

        if closed < open {
            Self::solve(solution, cur + ")", open, closed + 1, max);
        }
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        Solution::generate_parenthesis(3);
     }
 }