// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn full_justify(words: Vec<String>, max_len: i32) -> Vec<String> {
        let mut result = vec![];
        let max_len = max_len as usize;
    
        let mut line_len = 0;
        let mut from = 0;
    
        for idx in 0..words.len() {
            line_len += words[idx].len();
    
            if idx < words.len() - 1 {
                if line_len + words[idx + 1].len() + (idx - from) + 1 <= max_len {
                    continue;
                }
            }
    
            let mut line = String::with_capacity(max_len);
    
            if idx < words.len() - 1 {
                let word_count = idx - from + 1;
                let all_spaces = max_len - line_len;
    
                let mut eq_spaces = 0;
                let mut additional = 0;
                if word_count > 1 {
                    eq_spaces = all_spaces / (word_count - 1);
                    additional = all_spaces % (word_count - 1);
                }
    
                for word in &words[from..=idx] {
                    if !line.is_empty() {
                        let mut spaces = eq_spaces;
                        if additional > 0 {
                            spaces += 1;
                            additional -= 1;
                        }
    
                        (0..spaces).for_each(|_| line.push(' '));
                    }
    
                    line.push_str(word);
                }
            } else {
                for word in &words[from..] {
                    if !line.is_empty() {
                        line.push(' ');
                    }
                    line.push_str(word);
                }
            }
    
            while line.len() < max_len {
                line.push(' ');
            }
    
            result.push(line);
    
            from = idx + 1;
            line_len = 0;
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
     }
 }