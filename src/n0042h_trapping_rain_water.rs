// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(height.len());
        let mut solution = 0;
        let mut n: i32;
        let mut h_diff;

        for i in 1..height.len() {
            n = height[i-1] - height[i];

            if n > 0 {

                stack.push((i, n));

            } else if n < 0 {
                n = -n;
                loop {
                    match stack.pop() {
                        None => break,
                        Some((i_l, n_l)) => {
                            h_diff = n_l - n;
                            if h_diff > 0 {
                                solution += n * (i as i32 - i_l as i32);
                                stack.push((i_l, h_diff));
                                break
                            } else if h_diff == 0 {
                                solution += n_l * (i as i32 - i_l as i32);
                                break
                            } else {
                                solution += n_l * (i as i32 - i_l as i32);
                                n -= n_l;
                            }
                        }
                    }
                }
            }
        }
        solution as i32
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::trap(vec![6,4,2,0,3,2,0,3,1,4,5,3,2,7,5,3,0,1,2,1,3,4,6,8,1,3]), 83);
     }
 }