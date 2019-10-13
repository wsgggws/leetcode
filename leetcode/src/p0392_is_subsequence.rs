// 392. Is Subsequence
// Easy

// Given a string s and a string t, check if s is subsequence of t.

// You may assume that there is only lower case English letters in both s and t. t is potentially a very long (length ~= 500,000) string, and s is a short string (<=100).

// A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ace" is a subsequence of "abcde" while "aec" is not).

// Example 1:
// s = "abc", t = "ahbgdc"

// Return true.

// Example 2:
// s = "axc", t = "ahbgdc"

// Return false.

// Follow up:
// If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?

// Credits:
// Special thanks to @pbrother for adding this problem and creating all test cases.

pub struct Solution {}


impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let chars_s: Vec<char> = s.chars().collect();
        let chars_t: Vec<char> = t.chars().collect();
        let mut index_i = 0_usize;
        let mut index_j = 0_usize;
        while index_j < chars_t.len() && index_i < chars_s.len() {
            if chars_t[index_j] == chars_s[index_i] { index_i += 1; }
            index_j += 1_usize;
        }
        if index_i == chars_s.len() { true } else { false }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_subsequence_test() {
        assert_eq!(Solution::is_subsequence("abc".to_owned(), "ahbgdc".to_owned()), true);
        assert_eq!(Solution::is_subsequence("axe".to_owned(), "ahbgdc".to_owned()), false);
    }
}
