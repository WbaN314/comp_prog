// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m: usize = m as usize;
        let n: usize = n as usize;
        let mut tmp = vec![0; n+m];
        let mut tmp_idx: usize = 0;
        let mut a_idx: usize = 0;
        let mut b_idx: usize = 0;

        while a_idx < m || b_idx < n {
            if a_idx == m {
                tmp[tmp_idx] = nums2[b_idx];
                b_idx += 1;
                tmp_idx += 1;
                continue
            } else if b_idx == n {
                tmp[tmp_idx] = nums1[a_idx];
                tmp_idx += 1;
                a_idx += 1;
                continue
            }

            if nums1[a_idx] < nums2[b_idx] {
                tmp[tmp_idx] = nums1[a_idx];
                tmp_idx += 1;
                a_idx += 1;
            } else {
                tmp[tmp_idx] = nums2[b_idx];
                tmp_idx += 1;
                b_idx += 1
            }
        }

        for i in 0..tmp.len() {
            nums1[i as usize] = tmp[i];
        }
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