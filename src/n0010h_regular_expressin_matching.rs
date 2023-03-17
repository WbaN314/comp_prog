// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {

    pub fn is_match(s: String, p: String) -> bool {

        let s = s.as_bytes();
        let p = p.as_bytes();

        let tokens = Self::parse_regex(&p);
        println!("{tokens:?}");

        return Self::solve(&s, &tokens);
    }

    fn solve(s: &[u8], tokens: &Vec<&[u8]>) -> bool {
        return true 
    }

    // make token vector splitting regex at .* and <char>* expressions
    fn parse_regex(p: &[u8]) -> Vec<&[u8]> {
        let mut v: Vec<&[u8]> = Vec::new();
        let mut i_l= 0;
        let mut i_r = 0;
        loop {
            match p.get(i_r) {
                None => {
                    if i_l + 1 <= i_r {
                        v.push(&p[i_l..=i_r -1])
                    };
                    break;
                },
                Some(r) if r == &b'*' => {
                    if i_l + 2 <= i_r {
                        v.push(&p[i_l..=i_r - 2]);
                    }
                    v.push(&p[i_r - 1..=i_r]);
                    i_r += 1;
                    i_l = i_r;
                },
                _ => {
                    i_r += 1;
                }
            }
        }
        return v
    }
}
 
 // END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let s = String::from("aaa");
        let p = String::from(".*.*aaabbb.*aaa");
        assert_eq!(false, Solution::is_match(s, p));
    }
 }