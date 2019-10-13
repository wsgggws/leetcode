// 387. First Unique Character in a String
// Easy

// Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.

// Examples:

// s = "leetcode"
// return 0.

// s = "loveleetcode",
// return 2.
// Note: You may assume the string contain only lowercase letters.

pub struct Solution {}


use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut letters = HashMap::new();
        for ch in chars.iter() {
            let counter = letters.entry(ch).or_insert(0);
            *counter += 1;
        }
        for (index, ch) in chars.iter().enumerate() {
            if letters.get(&ch) == Some(&1) { return index as i32; }
        }
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_uniq_char_test() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("eettoo".to_string()), -1);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
}
