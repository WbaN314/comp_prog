// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn count_and_say(n: i32) -> String {
       match n {
            1 => return String::from("1"),
            n => {
                let mut new = String::new();
                let s = Self::count_and_say(n - 1);
                let chars: Vec<char> = s.chars().collect();
                let mut count = 0;
                let mut last: char = ' ';
                for i in 0..chars.len() {
                    if last == ' ' {
                        count = 1;
                        last = chars[i];
                    } else if chars[i] == last {
                        count += 1;
                    } else {
                        new.push_str(&count.to_string());
                        new.push_str(&last.to_string());
                        last = chars[i];
                        count = 1;
                    }
                };
                new.push_str(&count.to_string());
                new.push_str(&last.to_string());
                return new
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
        assert_eq!(Solution::count_and_say(1), String::from("1"))
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::count_and_say(2), String::from("11"))
     }
     #[test]
     fn test_3() {
        assert_eq!(Solution::count_and_say(3), String::from("21"))
     }
     #[test]
     fn test_4() {
        assert_eq!(Solution::count_and_say(4), String::from("1211"))
     }

 }