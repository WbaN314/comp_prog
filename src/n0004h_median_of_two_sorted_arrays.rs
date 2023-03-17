// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut solution = Vec::with_capacity(nums1.len() + nums2.len());

        let mut idx1 = 0;
        let mut idx2 = 0;

        for _ in 0..(nums1.len() + nums2.len()) {
            match (nums1.get(idx1), nums2.get(idx2)) {
                (Some(num1), Some(num2)) => {
                    if num1 < num2 {
                        solution.push(num1);
                        idx1 += 1;
                    } else {
                        solution.push(num2);
                        idx2 += 1;
                    }
                },
                (Some(num), None) | (None, Some(num)) => {
                    solution.push(num);
                    idx1 += 1;
                    idx2 += 1;
                }
                _ => ()
            }
        }

        if solution.len() % 2 == 0 {
            return (f64::from(*solution[solution.len() / 2 - 1]) + f64::from(*solution[solution.len() / 2])) / f64::from(2)
        } else {
            return f64::from(*solution[solution.len()/2])
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