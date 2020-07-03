// 72. Edit Distance
// Hard

// Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.

// You have the following 3 operations permitted on a word:

// Insert a character
// Delete a character
// Replace a character
// Example 1:

// Input: word1 = "horse", word2 = "ros"
// Output: 3
// Explanation: 
// horse -> rorse (replace 'h' with 'r')
// rorse -> rose (remove 'r')
// rose -> ros (remove 'e')
// Example 2:

// Input: word1 = "intention", word2 = "execution"
// Output: 5
// Explanation: 
// intention -> inention (remove 't')
// inention -> enention (replace 'i' with 'e')
// enention -> exention (replace 'n' with 'x')
// exention -> exection (replace 'n' with 'c')
// exection -> execution (insert 'u')

pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        for i in 1..=word1.len() {
            dp[i][0] = i as i32;
        }
        for j in 1..=word2.len() {
            dp[0][j] = j as i32;
        }

        let ch1: Vec<char> = word1.chars().collect();
        let ch2: Vec<char> = word2.chars().collect();

        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if ch1[i-1] == ch2[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = i32::min(
                        dp[i-1][j],
                        i32::min(dp[i][j-1], dp[i-1][j-1])
                    ) + 1;
                }
            }
        }
        dp[word1.len()][word2.len()]

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_distance_test() {
        assert_eq!(Solution::min_distance("horse".to_owned(), "ros".to_owned()), 3);
        assert_eq!(Solution::min_distance("intention".to_owned(), "execution".to_owned()), 5);
    }
}
