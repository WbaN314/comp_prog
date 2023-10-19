// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let chars = s.chars().collect::<Vec<char>>();
        let len = chars.len();

        let mut result = Vec::new();

        for i in 1..4 {
            if i >= len {
                break;
            }
            for j in i+1..i+4 {
                if j >= len {
                    break;
                }
                for k in j+1..j+4 {
                    if k >= len {
                        break;
                    }
                    if len - k > 3 {
                        continue;
                    }
                    let mut ip = String::new();
                    let mut num = 0;
                    for l in 0..i {
                        num = num * 10 + chars[l] as i32 - '0' as i32;
                        ip.push(chars[l]);
                    }
                    if num > 255 || (i > 1 && chars[0] == '0') {
                        continue;
                    }
                    ip.push('.');
                    num = 0;
                    for l in i..j {
                        num = num * 10 + chars[l] as i32 - '0' as i32;
                        ip.push(chars[l]);
                    }
                    if num > 255 || (j - i > 1 && chars[i] == '0') {
                        continue;
                    }
                    ip.push('.');
                    num = 0;
                    for l in j..k {
                        num = num * 10 + chars[l] as i32 - '0' as i32;
                        ip.push(chars[l]);
                    }
                    if num > 255 || (k - j > 1 && chars[j] == '0') {
                        continue;
                    }
                    ip.push('.');
                    num = 0;
                    for l in k..len {
                        num = num * 10 + chars[l] as i32 - '0' as i32;
                        ip.push(chars[l]);
                    }
                    if num > 255 || (len - k > 1 && chars[k] == '0') {
                        continue;
                    }
                    result.push(ip);
                }
            }
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