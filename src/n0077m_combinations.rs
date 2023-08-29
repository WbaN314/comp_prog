// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE


impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut slate = vec![false; n as usize];
        let mut solution = Vec::with_capacity(Self::c_nk(n, k));

        Self::helper(&mut solution, &mut slate, k, k);

        return solution;
    }

    fn c_nk(n: i32, k: i32) -> usize {
        let n: u64 = n as u64;
        let k: u64 = k as u64;
        let mut a: u64 = 1;
        let mut b: u64 = 1;

        for i in 1..k {
            a = a * (n - (k - i));
            b = b * i;
        }

            (a / b) as usize
    }  

    fn helper(solution: &mut Vec<Vec<i32>>, slate: &mut Vec<bool>, k: i32, orig_size: i32) {
        if k == 0 {
            let mut new = Vec::with_capacity(orig_size as usize);
            for i in 0..slate.len() {
                if slate[i] {
                    new.push(i as i32 + 1);
                }
            }
            solution.push(new);
        }
        else {
            for i in (0..slate.len()).rev() {
                if slate[i] {
                    break;
                } else {
                    slate[i] = true;
                    Self::helper(solution, slate, k - 1, orig_size);
                    slate[i] = false;
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