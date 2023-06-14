// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // should be binomial(n+m, n)
        // (n+m)! / n! (n+m - n)! = (n+m)! / n! m!
        let m = m - 1;
        let n = n - 1;
        Self::bin(n+m, m)
    }

    fn bin(n: i32, k: i32) -> i32 {

        let nk = n as u128 - k as u128;
        let n = n as u128;
        let k = k as u128;

        if nk > k {
            ((nk+1..=n).product::<u128>() / (1..=k).product::<u128>()) as i32
        } else {
            ((k+1..=n).product::<u128>() / (1..=nk).product::<u128>()) as i32
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