// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // if number of rows is 1, then just return the string as is
        if num_rows == 1 {
            return s;
        }

        // convert num_rows into usize once and for all
        let num_rows = num_rows as usize;

        // create a string with all space it will ever need
        // this make sure it will not need to be reallocated once created
        let mut out = String::with_capacity(s.len());

        // finally, read chars from `[&u8]` instead of `s.chars()` or collecting it as a vec
        // doing so instead of `[&u8]` reduces runtime from 58ms to 0ms
        // and using `.bytes()` instead of collecting it into a vec reduces memory from bottom 20% to top 20%
        let bytes = s.as_bytes();

        // loop over each line, because chars will be in order of line number
        for line in 0..num_rows as usize {

            // skip line - line n starts from the nth character
            // step_by - higher `num_rows` increases the gap between vertical columns
            (0..s.len()).skip(line).step_by(2 * num_rows - 2).for_each(|i| {

                // push the char of the vertical column
                out.push(bytes[i] as char);

                // now, push the char from the "zig zag" diagonal
                // the first and last line does not have this, so they can be ignored
                if line != 0 && line != num_rows - 1 {

                    // if there is no chars after the column, then it will be None
                    // this happens when the column is at the end of string
                    if let Some(&c) = bytes.get(i + (num_rows - 1 - line) * 2) {

                        out.push(c as char);

                    }
                }
            });

        }

        out
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