// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

// abc|de
// caebd
// entweder sub(abc) = cae && sub(de) = bd oder sub(de) = ca && sub(abc) = ebd

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {

        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let l = chars1.len();

        let mut dp = vec![vec![vec![None; l]; l]; l];
        
        for i in 0..l {
            for j in 0..l {
                dp[0][i][j] = Some(chars1[i] == chars2[j])
            }
        }

        for length in 2..=l {
            for i in 0..=l-length {
                for j in 0..=l-length {
                    Self::scramble(&mut dp, &chars1, &chars2, i, j, length);
                }
            }
        }
        
        return dp[l-1][0][0].unwrap()
    }

    fn scramble(dp: &mut Vec<Vec<Vec<Option<bool>>>> ,chars1: &Vec<char>, chars2: &Vec<char>, i: usize, j: usize, length: usize) -> bool {
        if let Some(is_scrambled) = dp[length-1][i][j] {
            return is_scrambled
        } else {

            for s in 1..length {

                let no_swap = Self::scramble(dp, chars1, chars2, i, j, s) && Self::scramble(dp, chars1, chars2, i+s, j+s, length - s);
                let swap = Self::scramble(dp, chars1, chars2, i, j+length-s, s) && Self::scramble(dp, chars1, chars2, i+s, j, length - s);
                let result = swap || no_swap;

                if result {
                    dp[length-1][i][j] = Some(true);
                    return true
                }

            }
            dp[length-1][i][j] = Some(false);
            return false
        };
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::is_scramble(String::from("abcde"), String::from("caebd")), false);
     }

     #[test]
     fn test_2() {
        assert_eq!(Solution::is_scramble(String::from("a"), String::from("a")), true);
     }

     #[test]
     fn test_3() {
        assert_eq!(Solution::is_scramble(String::from("great"), String::from("rgeat")), true);
     }
 }