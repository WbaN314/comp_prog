// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut s_idx = 0;
        let mut p_idx = 0;
        let mut match_i = 0;
        let mut star_i: i32= - 1;

        while s_idx < s.len() {
            if p_idx < p.len() && p.as_bytes()[p_idx] as char == '*' { // if * in p
                star_i = p_idx as i32; // set star_i to p_idx
                match_i = s_idx; // set match_i to s_idx
                p_idx += 1; // next p idx
            } else if p_idx < p.len()
                    && (s.as_bytes()[s_idx] == p.as_bytes()[p_idx] || p.as_bytes()[p_idx] as char =='?') { // if direct match 
                s_idx += 1; // increase p idx
                p_idx += 1; // increase s idx
            } else if star_i >= 0 { // if star has been encountered (and no direct match as ifs before didnt hit)
                match_i += 1; // increase match_i (what should be matched by *)
                s_idx = match_i; // set s_idx to char after match by *
                p_idx = (star_i + 1) as usize; // set p_idx to char after *
            } else { return false; }
        }
        while p_idx < p.len() && p.as_bytes()[p_idx] as char == '*' { // run through remaining p chars if they are *
            p_idx += 1;
        }
        return p_idx == p.len();
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::is_match(String::from("ab"), String::from("*a")), false);
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("aa")), true);
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
        assert_eq!(Solution::is_match(String::from("ccdd"), String::from("c*d")), true);
     }
     #[test]
     fn test_6() {
        assert_eq!(Solution::is_match(String::from("acdcb"), String::from("a*c?b")), false);
     }
     #[test]
     fn test_7() {
        assert_eq!(Solution::is_match(String::from("leetcode"), String::from("*e*t?d*")), false);
     }
     #[test]
     fn test_8() {
        assert_eq!(Solution::is_match(String::from("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb"), String::from("**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb")), false);
     }
 }