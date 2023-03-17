// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {

    pub fn is_match(s: String, p: String) -> bool {

        let s = s.as_bytes();
        let p = p.as_bytes();

        let tokens = Self::token_vec(&p);

        return Self::solve(&s, &p, &tokens);
    }

    fn solve(s: &[u8], p: &[u8], tokens: &Vec<(usize, usize)) -> bool {
        while let 
    }

    fn get_potential_match(s: &[u8], token: (usize, usize), start: usize) -> usize {

    }

    // make token vector splitting regex at .* and <char>* expressions
    fn token_vec(p: &[u8]) -> Vec<(usize, usize)> {
        let mut v: Vec<(usize, usize)> = Vec::new();
        let mut i_l= 0;
        let mut i_r = 0;
        loop {
            match p.get(i_r) {
                None => {
                    if i_l <= i_r - 1 {
                        v.push((i_l, i_r -1))
                    };
                    break;
                },
                Some(r) if r == &b'*' => {
                    if i_l <= i_r - 2 {
                        v.push((i_l, i_r - 2));
                    }
                    // v.push((i_r - 1, i_r)); keeping the * tokens out of the vec is enough for this problem
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
        println!("{:?}", Solution::token_vec("aaaaa*aaaaa.*ddddd".as_bytes()));
        // assert_eq!(false ,Solution::is_match("a".to_string(), "aa*aaaa.*xc".to_string()));
    }
 }