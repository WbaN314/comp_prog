// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

use std::cmp::min;
impl Solution {

    pub fn max_area(height: Vec<i32>) -> i32 {

        if height.len() == 0 {return 0}

        let mut left = 0;
        let mut right = height.len() - 1;

        let mut best = 0;
        let mut current = 0;
        while left < right {
            current = (right - left) as i32 * min(height[left], height[right]);

            if current > best {
                best = current;
            }

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        };

        best
    }
}
 
// END SUBMISSION CODE
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(49, Solution::max_area(vec![1,8,6,2,5,4,8,3,7]))
    }
}