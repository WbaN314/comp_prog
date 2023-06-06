// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

use std::collections::HashSet;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut a: [i16; 10001] = [0; 10001];
        let mut points = HashSet::new();
        let mut solution = Vec::new();

        for v in intervals {
            if v[0] == v[1] {
                points.insert(v[0]);
            } else {
                a[v[0] as usize] += 1;
                a[v[1] as usize] -= 1;
            }
        }

        let mut current = 0;
        for i in 0..a.len() {
            current += a[i];
            a[i] = current;
        }


        let mut start = 0;
        let mut end = 0;
        for i in 0..a.len() - 1 {
            if a[i] > 0 && a[i+1] == 0 {
                end = i + 1;
                solution.push(vec![start as i32, end as i32]);
            } else if a[i] == 0 && a[i+1] > 0 {
                start = i + 1; 
            }
        }

        for i in points {
            if a[i as usize] == 0 && (i == 0 || a[i as usize - 1] == 0) {
                solution.push(vec![i as i32, i as i32]);
            }
        }

        solution
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