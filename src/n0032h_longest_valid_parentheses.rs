// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut stack: Vec<i32> = vec![-1];
        let mut result = 0;

        for i in 0..s.len() {
            match s[i] {
                '(' => {
                    stack.push(i as i32);
                },
                _ => {
                    stack.pop();
                    if stack.is_empty() {
                        stack.push(i as i32);
                    } else {
                        result = result.max(i as i32 - stack.last().unwrap());
                    }
                }
            }
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
        let s = String::from("(()");
        assert_eq!(Solution::longest_valid_parentheses(s),2);
     }
     #[test]
     fn test_2() {
        let s = String::from("()()");
        assert_eq!(Solution::longest_valid_parentheses(s),4);
     }
     #[test]
     fn test_3() {
        let s = String::from(")()()");
        assert_eq!(Solution::longest_valid_parentheses(s),4);
     }
     #[test]
     fn test_4() {
        let s = String::from("((()))");
        assert_eq!(Solution::longest_valid_parentheses(s),6);
     }
     #[test]
     fn test_5() {
        let s = String::from("())))()())");
        assert_eq!(Solution::longest_valid_parentheses(s),4);
     }
     #[test]
     fn test_6() {
        let s = String::from("(()()");
        assert_eq!(Solution::longest_valid_parentheses(s),4);
     }
 }