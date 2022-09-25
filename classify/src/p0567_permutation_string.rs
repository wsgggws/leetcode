// 567. Permutation in String
// Medium

// Given two strings s1 and s2, write a function to return true if s2 contains the permutation of s1. In other words, one of the first string's permutations is the substring of the second string.



// Example 1:

// Input: s1 = "ab" s2 = "eidbaooo"
// Output: True
// Explanation: s2 contains one permutation of s1 ("ba").
// Example 2:

// Input:s1= "ab" s2 = "eidboaoo"
// Output: False


// Constraints:

// The input strings only contain lower case letters.
// The length of both given strings is in range [1, 10,000].

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let hash_map_s1 = Solution::get_hash_map(&s1[..]);
        for i in 0..=s2.len() - s1.len() {
            if hash_map_s1 == Solution::get_hash_map(&s2[i..i+s1.len()]) {
                return true;
            }
        }
        false
    }

    fn get_hash_map(s1: &str) -> HashMap<char, usize> {
        let mut hash_map: HashMap<char, usize> = HashMap::new();
        for ch in s1.chars() {
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
    fn check_inclusion_test() {
        assert_eq!(Solution::check_inclusion("ab".to_owned(), "eidbaooo".to_owned()), true);
        assert_eq!(Solution::check_inclusion("ab".to_owned(), "eidboaoo".to_owned()), false);
        assert_eq!(Solution::check_inclusion("adc".to_owned(), "dcda".to_owned()), true);
        assert_eq!(Solution::check_inclusion("ab".to_owned(), "a".to_owned()), false);
        assert_eq!(Solution::check_inclusion("ab".to_owned(), "aa".to_owned()), false);
        assert_eq!(Solution::check_inclusion("ab".to_owned(), "ab".to_owned()), true);
    }
}
