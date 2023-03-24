// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // Early exit for empty digits
        if digits.is_empty() {
            return vec![];
        }

        // Set up dictionary to map keys to characters
        let keys = [
            vec!["a", "b", "c"],
            vec!["d", "e", "f"],
            vec!["g", "h", "i"],
            vec!["j", "k", "l"],
            vec!["m", "n", "o"],
            vec!["p", "q", "r", "s"],
            vec!["t", "u", "v"],
            vec!["w", "x", "y", "z"],
        ];

        // Do a fold to iteratively branch out the solution with
        // the possible characters of each digit

        digits
            .chars()
            .map(|d| d.to_digit(10).unwrap() - 2)
            .fold(vec![String::new()], |acc, i| {
                keys[i as usize]
                    .iter()
                    .flat_map(|c| acc.iter().map(move |s| s.clone() + c))
                    .collect()
            })
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