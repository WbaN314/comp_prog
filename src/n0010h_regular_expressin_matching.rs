// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {

    pub fn is_match(s: String, p: String) -> bool {

        let s = s.as_bytes();
        let p = p.as_bytes();

        let tokens = Self::parse_regex(&p);

        return Self::solve(&s, &tokens);
    }

    fn solve(s: &[u8], tokens: &[&[u8]]) -> bool {

        // get next non * token, if there is none, resolve tail
        let mut token_index = 0;
        let mut star_chars: Vec<u8> = Vec::new();
        let token = loop {
            match tokens.get(token_index) {
                None => return Self::check_between(s.len(), &star_chars, s),
                Some([char, star]) if star == &b'*' => {
                    star_chars.push(*char);
                    token_index += 1;
                }
                Some(token) => break *token
            }
        };

        // string is too short to match token, return false
        if s.len() < token.len() {
            return false
        }


        'outer: for i in 0..=s.len() - token.len() {
            for j in 0..token.len() {
                // next match
                if s[i + j] != token[j] && token[j] != b'.' {
                    continue 'outer
                }
            }
                
            // if found, check if in between * tokens are sufficient
            if !Self::check_between(i, &star_chars, s) {
                return false
            }

            // recursion with new s and tokens
            let solution = Self::solve(&s[i + token.len()..], &tokens[token_index + 1 ..]);
            if solution {
                return true
            } else {
                continue
            }   
        }

        return false 
    }

    fn check_between(i: usize, star_chars: &Vec<u8>, s: &[u8]) -> bool {
        let mut star_char_index = 0;
        let mut string_index = 0;
        loop {
        
            // between * tokens were sufficient
            if string_index == i {
                return true
            }
    
            // between * tokens are not sufficient
            if star_char_index == star_chars.len() {
                return false
            }

            if star_chars[star_char_index] == b'.' {
                return true
            }
    
            // forward either star tokens or string
            if star_chars[star_char_index] == s[string_index] {
                string_index += 1;
            } else {
                star_char_index += 1;
            }
        }
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

 // above works, but there is probably a much nicer implementation possible using slice patterns
 
// fn is_match(s: &[u8], p: &[u8]) -> bool {
//     match (p, s) {
//         ([x, b'*', subp..], [y, subs..]) if *x == b'.' || x == y => is_match(subs, p),
//         ([_, b'*', subp..], _) => is_match(s, subp),
//         ([x, subp..], [y, subs..]) if *x == b'.' || x == y => is_match(subs, subp),
//         ([], s) => s.is_empty(),
//         _ => false,
//     }
// }
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let s = String::from("aaa");
        let p = String::from(".*.*aaabbb.*aaa");
        assert_eq!(false, Solution::is_match(s, p));
    }
    #[test]
     fn test_2() {
        let s = String::from("aaa");
        let p = String::from(".*");
        assert_eq!(true, Solution::is_match(s, p));
    }
    #[test]
     fn test_3() {
        let s = String::from("aaabbb");
        let p = String::from("a*bbb");
        assert_eq!(true, Solution::is_match(s, p));
    }
    #[test]
     fn test_4() {
        let s = String::from("aaabbbcxxxlol");
        let p = String::from("a*bbbc.*lol");
        assert_eq!(true, Solution::is_match(s, p));
    }
    #[test]
     fn test_5() {
        let s = String::from("asdfghjklö");
        let p = String::from("as*g*ö");
        assert_eq!(false, Solution::is_match(s, p));
    }
    #[test]
     fn test_6() {
        let s = String::from("mississippi");
        let p = String::from("mis*is*ip*.");
        assert_eq!(true, Solution::is_match(s, p));
    }
    #[test]
     fn test_7() {
        let s = String::from("a");
        let p = String::from(".");
        assert_eq!(true, Solution::is_match(s, p));
    }
    #[test]
     fn test_8() {
        let s = String::from("ba");
        let p = String::from(".c");
        assert_eq!(false, Solution::is_match(s, p));
    }
 }