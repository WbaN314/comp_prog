// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE
use std::collections::HashMap;

impl Solution 
{
    pub fn min_window(s: String, t: String) -> String 
    {
        let ss = s.as_bytes();
        
        let mut freq: HashMap<u8,i32> = t
            .bytes()
            .fold(HashMap::new(), |mut f,ch| { *f.entry(ch).or_default() += 1; f });
        
        let mut miss      = freq.len() as i32; 
        let (mut l, mut r)  = (0, 0);
        let (mut wl, mut wr) = (0, s.len()+1);
        
        loop
        {
            if r < ss.len() && miss > 0
            {
                if let Some(f) = freq.get_mut(&ss[r])
                {
                    *f -= 1;
                    if *f == 0 { miss -= 1; }
                }
                r += 1;
            }

            else if l < ss.len() && miss <= 0
            {
                if r-l < wr-wl { wl = l; wr = r; }

                if let Some(f) = freq.get_mut(&ss[l])
                {
                    if *f == 0 { miss += 1; }
                    *f += 1;
                }
                l += 1;
            }
            else { break; }
        }
        
        return if wr <= ss.len() { s[wl..wr].to_string() } else { String::new() };
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
     }
 }