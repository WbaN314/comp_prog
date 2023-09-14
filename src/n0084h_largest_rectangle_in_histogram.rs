use std::mem::MaybeUninit;

// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut best = 0;
        let mut stack = Vec::with_capacity(heights.len());
        let mut right_smaller_values = Vec::with_capacity(heights.len());

        // ok as all values will be set by for loop below from right to left
        unsafe {
            right_smaller_values.set_len(heights.len())
        }
        for i in (0..heights.len()).rev() {
            right_smaller_values[i] = if let Some(v) = Self::get_index(&mut stack, &heights, i) {v - 1} else {heights.len() - 1}
        }
        stack.clear();

        let mut left;
        let mut right;
        for i in 0..heights.len() {
            left = if let Some(v) = Self::get_index(&mut stack, &heights, i) {v + 1} else {0};
            right = right_smaller_values[i];
            best = best.max((right - left + 1) * heights[i] as usize);
        } 
        best as i32
    }

    fn get_index(stack: &mut Vec<usize>, heights: &Vec<i32>, current: usize) -> Option<usize> {
        loop {
            match stack.last() {
                None => {
                    stack.push(current);
                    return None
                }
                Some(&v) => {
                    if heights[v] < heights[current] {
                        stack.push(current);
                        return Some(v)
                    } else {
                        stack.pop();
                    }
                }
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
     }
 }