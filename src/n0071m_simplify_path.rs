// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();

        for s in path.split("/") {
            match s {
                "" | "." => continue,
                ".." => {
                    stack.pop();
                }
                _ => {
                    stack.push(s);
                }
            }
        }

        let mut result = String::new();
        for s in stack {
            result.push('/');
            result.push_str(s);
        }
        
        if result.is_empty() { String::from("/") } else { result }
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