// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        
        let mut start = new_interval[0];
        let mut end = new_interval[1];
        for i in 0..intervals.len() {
            if  (intervals[i][1] >= start) && (intervals[i][0] <= end) {
                start = start.min(intervals[i][0]);
                end = end.max(intervals[i][1]);
                intervals[i][0] = -1;
            }
        }

        let mut new_intervals = Vec::new();
        let mut inserted = false;
        for i in intervals {
            if i[0] == -1 {
                continue
            }
            if !inserted && i[0] > start {
                new_intervals.push(vec![start, end]);
                inserted = true;
            }
            new_intervals.push(i)
        }

        if !inserted {
            new_intervals.push(vec![start, end]);
        }

        new_intervals
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