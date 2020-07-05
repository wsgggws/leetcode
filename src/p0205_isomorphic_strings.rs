// 205. Isomorphic Strings
// Easy

// Given two strings s and t, determine if they are isomorphic.

// Two strings are isomorphic if the characters in s can be replaced to get t.

// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character but a character may map to itself.

// Example 1:

// Input: s = "egg", t = "add"
// Output: true
// Example 2:

// Input: s = "foo", t = "bar"
// Output: false
// Example 3:

// Input: s = "paper", t = "title"
// Output: true
// Note:
// You may assume both s and t have the same length.

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut hash_map: HashMap<char, char> = HashMap::new();
        let ch_s: Vec<char> = s.chars().collect();
        let ch_t: Vec<char> = t.chars().collect();

        for i in 0..s.len() {
            match hash_map.get(&ch_s[i]) {
                Some(&c) => {
                    if c != ch_t[i] {
                        return false;
                    }
                },
                None => {
                    hash_map.insert(ch_s[i], ch_t[i]);
                }
            }
        }

        let mut hash_map2 = HashMap::new();
        for (k, v) in hash_map.iter() {
            hash_map2.insert(v, k);
        }
        hash_map2.len() == hash_map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_isomorphic_test() {
        assert_eq!(Solution::is_isomorphic("ab".to_owned(), "aa".to_owned()), false);
        assert_eq!(Solution::is_isomorphic("egg".to_owned(), "add".to_owned()), true);
        assert_eq!(Solution::is_isomorphic("foo".to_owned(), "bar".to_owned()), false);
        assert_eq!(Solution::is_isomorphic("paper".to_owned(), "title".to_owned()), true);
    }
}
