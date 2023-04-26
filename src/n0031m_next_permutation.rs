// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (1..nums.len()).rev() {
            if nums[i-1] < nums[i] {
                let tmp = i + nums[i..].partition_point(|&x| x > nums[i-1]) - 1;
                nums.swap(i-1, tmp);
                nums[i..].sort_unstable();
                return
            }
        }
        nums.reverse()
    }
}

// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let mut a = vec![2, 3, 1];
        Solution::next_permutation(&mut a);
        assert_eq!(a, vec![3,1,2]);
     }
     #[test]
     fn test_2() {
        let mut a = vec![3, 2, 1];
        Solution::next_permutation(&mut a);
        assert_eq!(a, vec![1,2,3]);
     }
     #[test]
     fn test_3() {
        let mut a = vec![1, 2, 3];
        Solution::next_permutation(&mut a);
        assert_eq!(a, vec![1,3,2]);
     }
 }