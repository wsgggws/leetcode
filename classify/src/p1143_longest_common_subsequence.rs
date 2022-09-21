// 1143. Longest Common Subsequence
// Medium

// Given two strings text1 and text2, return the length of their longest common subsequence.

// A subsequence of a string is a new string generated from the original string with some characters(can be none) deleted without changing the relative order of the remaining characters. (eg, "ace" is a subsequence of "abcde" while "aec" is not). A common subsequence of two strings is a subsequence that is common to both strings.

// If there is no common subsequence, return 0.

// Example 1:

// Input: text1 = "abcde", text2 = "ace"
// Output: 3
// Explanation: The longest common subsequence is "ace" and its length is 3.
// Example 2:

// Input: text1 = "abc", text2 = "abc"
// Output: 3
// Explanation: The longest common subsequence is "abc" and its length is 3.
// Example 3:

// Input: text1 = "abc", text2 = "def"
// Output: 0
// Explanation: There is no such common subsequence, so the result is 0.

// Constraints:

// 1 <= text1.length <= 1000
// 1 <= text2.length <= 1000
// The input strings consist of lowercase English characters only.<Paste>

pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let char1: Vec<char> = text1.chars().collect();
        let char2: Vec<char> = text2.chars().collect();

        let (n1, n2) = (text1.len(), text2.len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n2 + 1usize]; n1 + 1usize];

        for i in 1..=n1 {
            for j in 1..=n2 {
                if char1[i - 1] == char2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = i32::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[n1][n2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_subsequence_test() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_owned(), "ace".to_owned()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_owned(), "abc".to_owned()),
            3
        );
    }
}
