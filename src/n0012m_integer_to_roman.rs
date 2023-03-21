use std::vec;

// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
    
        let mut solution: Vec<char> = Vec::new();

        solution.append(&mut match num / 1000 {
            0 => vec![],
            1 => vec!['M'],
            2 => vec!['M', 'M'],
            3 => vec!['M', 'M', 'M'],
            _ => panic!("Out of range")
        });
        num = num % 1000;
        solution.append(&mut Self::helper(num / 100  , 'C', 'D', 'M'));
        num = num % 100;
        solution.append(&mut Self::helper(num / 10 , 'X', 'L', 'C'));
        num = num % 10;
        solution.append(&mut Self::helper(num, 'I', 'V', 'X'));

        solution.into_iter().collect::<String>()
    }

    fn helper(num: i32, i: char, v: char, x: char) -> Vec<char> {
        match num {
            0 => vec![],
            1 => vec![i],
            2 => vec![i, i],
            3 => vec![i, i, i],
            4 => vec![i, v],
            5 => vec![v],
            6 => vec![v, i],
            7 => vec![v, i, i],
            8 => vec![v, i, i, i],
            9 => vec![i, x],
            _ => panic!("Wrong number for helper")
        }
    }
}
 
 // END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        assert_eq!(Solution::int_to_roman(3), String::from("III"));
     }
     fn test_2() {
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
     }
     fn test_3() {
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
     }
 }