// 242. Valid Anagram
// Easy

// Given two strings s and t , write a function to determine if t is an anagram of s.

// Example 1:

// Input: s = "anagram", t = "nagaram"
// Output: true
// Example 2:

// Input: s = "rat", t = "car"
// Output: false
// Note:
// You may assume the string contains only lowercase alphabets.

// Follow up:
// What if the inputs contain unicode characters? How would you adapt your solution to such case?

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        Solution::get_hash_map(s) == Solution::get_hash_map(t)
    }

    fn get_hash_map(s: String) -> HashMap<char, usize> {
        let mut hash_map = HashMap::new();
        for ch in s.chars() {
            let counter = hash_map.entry(ch).or_insert(0);
            *counter += 1;
        }
        hash_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram_test() {
        assert_eq!(Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()), true);
        assert_eq!(Solution::is_anagram("rat".to_owned(), "car".to_owned()), false);
    }
}
