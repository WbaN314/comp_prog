// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char>= s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();

        if s1.len() + s2.len() != s3.len() {
            return false
        }

        let mut mem = vec![vec![vec![None; s3.len()]; s2.len()]; s1.len()];

        Self::progress(&s1, &s2, &s3, 0, 0, 0, &mut mem)
    }

    fn progress(s1: &Vec<char>, s2: &Vec<char>, s3: &Vec<char>, p1: usize, p2: usize, p3: usize, mem: &mut Vec<Vec<Vec<Option<bool>>>>) -> bool {
        if p3 == s3.len() {
            return p1 == s1.len() && p2 == s2.len()
        };

        if let Some(v1) = mem.get(p1) {
            if let Some(v2) = v1.get(p2) {
                if let Some(v3) = v2.get(p3) {
                    if let Some(v4) = v3 {
                        return *v4
                    } 
                }
            }
        }
        
        if p1 < s1.len() && s3[p3] == s1[p1] {
            if Self::progress(s1, s2, s3, p1 + 1, p2, p3 + 1, mem) {
                if p1 < s1.len() && p2 < s2.len() {
                    mem[p1][p2][p3] = Some(false)
                }
                return true
            }
        };
        if p2 < s2.len() && s3[p3] == s2[p2] {
            if Self::progress(s1, s2, s3, p1, p2 + 1, p3 + 1, mem) {
                if p1 < s1.len() && p2 < s2.len() {
                    mem[p1][p2][p3] = Some(false)
                }
                return true
            }
        };

        if p1 < s1.len() && p2 < s2.len() {
            mem[p1][p2][p3] = Some(false)
        }
        false
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let s3 = "hewolrllod".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), true);
     }

     #[test]
     fn test_2() {
        let s1 = "hello".to_string();
        let s2 = "worll".to_string();
        let s3 = "hewolrllod".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), false);
     }

     #[test]
     fn test_3() {
        let s1 = "abababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string();
        let s3 = "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string();
        let s2 = "babababababababababababababababababababababababababababababababababababababababababababababababaaaba".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), false);
     }
 }