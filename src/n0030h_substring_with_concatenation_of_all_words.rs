// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE


// It works but is ugly af, too lazy to clean up
// Basically maintain a hashmap of words to arrays where each value in the array represents a offset up until word_length where it rolls over again
// Then just go through s char by char and adapt the respective offset array entry for what falls of and what is added
use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, mut words: Vec<String>) -> Vec<i32> {

        // Create a map of char bytes that represent a word to a vec that represents
        // the number of times that word appears 
        let s = s.as_bytes();
        let words: Vec<&[u8]> = words.iter_mut().map(|word| word.as_bytes()).collect();
        let word_length = words[0].len();
        let words_number = words.len();
        let mut words_map: HashMap::<&[u8], Vec<usize>> = HashMap::new();
        for word in words {
            words_map.entry(word)
                .and_modify(|e| e[0] += 1)
                .or_insert({
                    let mut v = vec![0; word_length + 1];
                    v[0] += 1;
                    v
                });
        };

        let mut solution: Vec<i32> = Vec::new();

        let mut matched_words = vec![0; word_length];
        for i in 0..word_length {
            let tmp = Self::find_substring_initial(&s[i..], words_number, word_length, i, &mut words_map);
            matched_words[i] = tmp;
            if tmp == words_number as i32 {
                solution.push(i as i32);
            }
            if tmp == -1 {
                break
            }
        }

        let mut offset = 0;

        while s.len() - offset >= word_length * (words_number + 1) {
            let shift = offset % word_length;
            Self::find_substring_step(&s[offset..], words_number, word_length, shift, &mut words_map, &mut matched_words);
            if matched_words[shift] == words_number as i32 {
                solution.push((offset + word_length) as i32);
            }
            offset += 1;
        }

        solution
    }

    fn find_substring_initial<'a>(s: &'a[u8], words_number: usize, word_length: usize, shift: usize, words_map: &mut HashMap::<&'a[u8], Vec<usize>>) -> i32 {
        if s.len() < words_number * word_length {
            return -1
        }
        
        let mut count = 0;
        for i in 0..words_number {
            words_map.entry(&s[(i * word_length) as usize .. ((i+1) * word_length) as usize])
                .and_modify(|v| {
                    v[shift + 1] += 1;
                    if v[shift + 1] <= v[0] {
                        count += 1;
                    }
            });         
        }
        count
    }

    fn find_substring_step<'a>(s: &'a[u8], words_number: usize, word_length: usize, shift: usize, words_map: &mut HashMap::<&'a[u8], Vec<usize>>, matched_words: &mut Vec<i32>) {
        if s.len() < (words_number + 1) * word_length {
            matched_words[shift] = -1;
        }
        match words_map.get_mut(&s[0..word_length]) {
            Some(v) => {
                v[shift + 1] -= 1;
                if v[shift +1] < v[0] {
                    matched_words[shift] -= 1;
                }
            },
            _ => ()
        };

        words_map.entry(&s[(words_number * word_length) as usize .. ((words_number + 1) * word_length) as usize])
            .and_modify(|v| {
                v[shift + 1] += 1;
                if v[shift + 1] <= v[0] {
                    matched_words[shift] += 1;
                }
            });
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
        let s = String::from("barfoothefoobarman");
        let t = vec![String::from("foo"),
            String::from("bar")];
        assert_eq!(Solution::find_substring(s, t), vec![0,9]);
     }

     #[test]
     fn test_2() {
        let s = String::from("wordgoodgoodgoodbestword");
        let t = vec![String::from("word"),
            String::from("good"),
            String::from("best"),
            String::from("good"),
            ];
        assert_eq!(Solution::find_substring(s, t), vec![8]);
     }
 }