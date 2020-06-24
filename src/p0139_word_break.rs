// 139. Word Break
// Medium
// Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, determine if s can be segmented into a space-separated sequence of one or more dictionary words.

// Note:

// The same word in the dictionary may be reused multiple times in the segmentation.
// You may assume the dictionary does not contain duplicate words.
// Example 1:

// Input: s = "leetcode", wordDict = ["leet", "code"]
// Output: true
// Explanation: Return true because "leetcode" can be segmented as "leet code".
// Example 2:

// Input: s = "applepenapple", wordDict = ["apple", "pen"]
// Output: true
// Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
//              Note that you are allowed to reuse a dictionary word.
// Example 3:

// Input: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
// Output: false

pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // dp[i] 表示0..=i是否可以使用word_dict去拼接
        let mut dp: Vec<bool> = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 0..s.len() {
            for j in 0..word_dict.len() {
                if dp[i] == true && i+word_dict[j].len() <= s.len() && s[i..i+word_dict[j].len()] == word_dict[j] {
                    dp[i+word_dict[j].len()] = true;
                    if dp[s.len()] == true {
                        // 优化，找到了就立马结束
                        return true;
                    }
                }
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_break_test() {
        assert_eq!(
            Solution::word_break(
                "leetcode".to_owned(),
                vec!["leet".to_owned(), "code".to_owned()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break(
                "applepenapple".to_owned(),
                vec!["apple".to_owned(), "pen".to_owned()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break(
                "catsandog".to_owned(),
                vec![
                    "cats".to_owned(),
                    "dog".to_owned(),
                    "sand".to_owned(),
                    "and".to_owned(),
                    "cat".to_owned()
                ]
            ),
            false
        );
    }
}
