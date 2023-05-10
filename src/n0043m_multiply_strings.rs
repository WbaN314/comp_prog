// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {

        // special case 0 string
        if num1 == String::from("0") || num2 == String::from("0") {
            return String::from("0");
        }
        
        // convert to vector of numbers with values from 0 to 9
        // digit d in i-th position relates to (i+1) * d as original value in string (it is basically reversed)
        let num1: Vec<u32> = num1.chars().map(|c| c.to_digit(10).unwrap()).rev().collect();
        let num2: Vec<u32> = num2.chars().map(|c| c.to_digit(10).unwrap()).rev().collect();

        // create vector with maximum reachable length all initialized to 0
        let mut solution = vec![0; num1.len() + num2.len()];

        // multiply two number vectors
        for (i1, v1) in num1.iter().enumerate() {
            for (i2, v2) in num2.iter().enumerate() {
                let (x, c) = ((v1 * v2) % 10, (v1 * v2) / 10);
                let mut i = i1 + i2;

                solution[i] += x;
                solution[i+1] += c;

                // do carry over if single digit is >9
                while solution[i] > 9 {
                    solution[i+1] += solution[i] / 10;
                    solution[i] %= 10;
                    i += 1;
                }
            }
        }

        // determine trailing 0s
        let mut up_to: usize = 0;
        for i in (0..solution.len()).rev() {
            if solution[i] != 0 {
                up_to = i+1;
                break
            }
        }
        
        // construct String again
        solution[0..up_to].iter().rev().map(|n| char::from_digit(*n, 10).unwrap()).collect()
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::multiply(String::from("123"), String::from("456")), String::from("56088"));
     }
 }