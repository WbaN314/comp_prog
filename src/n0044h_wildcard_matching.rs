// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let mut p_stack: Vec<Vec<char>> = Vec::new();
        let mut l = 0;
        for r in 0..p.len() {
            match p[r] {
                '*' => {
                    if r > l {
                        p_stack.push(p[l..r].to_vec());
                    }
                    if p_stack.last() != Some(&vec!['*']) {
                        p_stack.push(vec!['*']);
                    }
                    l = r + 1;
                },
                _ if r == p.len() - 1 && p[l..].len() > 0 => {
                    p_stack.push(p[l..].to_vec());
                }
                _ => ()
            }
        }
        p_stack = p_stack.into_iter().rev().collect();

        // println!("{p_stack:?}");

        Self::backtrack_solution(&s[..], &mut p_stack)
    }

    fn backtrack_solution(s: &[char], p_stack: &mut Vec<Vec<char>>) -> bool {
        match p_stack.pop() {

            Some(x) if x == vec!['*'] => {
                for i in 0..=s.len() {
                    if Self::backtrack_solution(&s[i..], p_stack) {
                        return true
                    }
                }
                p_stack.push(x);
                return false
            },
            
            Some(x) if s.len() < x.len() => {
                p_stack.push(x);
                return false
            },

            Some(x) if s.len() == 0 => {
                p_stack.push(x);
                return false
            },

            Some(x) => {
                for i in 0..x.len() {
                    if x.get(i) != s.get(i) && x[i] != '?' {
                        p_stack.push(x);
                        return false
                    }
                }
                if Self::backtrack_solution(&s[x.len()..], p_stack) {
                    return true
                }
                p_stack.push(x);
                return false
            },

            None if s.len() > 0 => {
                return false
            },

            None => {
                return true
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
        assert_eq!(Solution::is_match(String::from("aabbccdd"), String::from("*")), true);
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::is_match(String::from("aabbccdd"), String::from("a")), false);
     }
     #[test]
     fn test_3() {
        assert_eq!(Solution::is_match(String::from("aabbccdd"), String::from("a*")), true);
     }
     #[test]
     fn test_4() {
        assert_eq!(Solution::is_match(String::from("aabbccdd"), String::from("a*b")), false);
     }
     #[test]
     fn test_5() {
        assert_eq!(Solution::is_match(String::from("aabbccdd"), String::from("a*c**d")), true);
     }
     #[test]
     fn test_6() {
        assert_eq!(Solution::is_match(String::from("acdcb"), String::from("a*c?b")), false);
     }
     #[test]
     fn test_7() {
        assert_eq!(Solution::is_match(String::from("leetcode"), String::from("*e*t?d*")), false);
     }
 }