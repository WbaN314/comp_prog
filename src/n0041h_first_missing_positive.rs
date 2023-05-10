// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {

        let mut value;
        let mut read_from: usize = 0;
        let mut write_to: usize = 0;
        let mut read_stored: bool = false;

        // replace all 0 with -1 to use 0 as "has seen" indicator later on
        for i in 0..nums.len() {
            if nums[i] == 0 {
                nums[i] = -1;
            }
        }

        while read_from < nums.len() && write_to < nums.len() {

            // println!("{nums:?} {read_from} {write_to}");

            value = if read_stored {
                read_stored = false;
                nums[write_to] - 1
            } else {
                nums[read_from] - 1
            };

            if value == write_to as i32 {
                nums[write_to] = 0;
                while nums.get(write_to) == Some(&0) {
                    write_to += 1;
                }
                read_from += 1;
            } else if value < write_to as i32 || value >= nums.len() as i32 {
                read_from += 1;
            } else {
                nums[write_to] = nums[value as usize];
                nums[value as usize] = 0;
                read_stored = true;
            }
            
        }

        // println!("{nums:?} {read_from} {write_to}");

        (write_to + 1) as i32
    }
}

// 0 2 1 -> set on
// 0 2 1
// -1 2 1


// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5]), 6);
     }
     #[test]
     fn test_2() {
        assert_eq!(Solution::first_missing_positive(vec![5, 4, 3, 2, 1]), 6);
     }
     #[test]
     fn test_3() {
        assert_eq!(Solution::first_missing_positive(vec![3,4,-1,1]), 2);
     }
 }