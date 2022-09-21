// 91. Decode Ways
// Medium

// A message containing letters from A-Z is being encoded to numbers using the following mapping:

// 'A' -> 1
// 'B' -> 2
// ...
// 'Z' -> 26
// Given a non-empty string containing only digits, determine the total number of ways to decode it.

// Example 1:

// Input: "12"
// Output: 2
// Explanation: It could be decoded as "AB" (1 2) or "L" (12).
// Example 2:

// Input: "226"
// Output: 3
// Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).

pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // dp[i] 表示0..i 的组合数量
        // 则根据[i-2..i] 进行讨论，并注意后面为0的情况
        // 00 => dp[i] = 0
        // 01 <= num <= 09 , => dp[i] = dp[i-1], 即只能最后一位单独表示字母
        // num > 26
        //      1. num % 10 == 0  => dp[i] = 0, 显然无法表示
        //      2. num % 10 != 0 => dp[i] = dp[i-1], 即只能最后一位单独表示字母
        // 10 <= num <= 26
        //      1. num % 10 == 0  => dp[i] = dp[i-2], 即可能最后两位来表示字母，如20
        //      2. num % 10 != 0  => dp[i] = dp[i-1] + dp[i-2], 如22, 即可以表示dp[i-2],22, dp[i-1], 2
        let mut dp: Vec<i32> = vec![0; s.len()+1];
        dp[0] = 1;
        // NOTE, 特别注意开头字母为零
        if s[0..1].parse::<u32>().unwrap() == 0 {
            dp[1] = 0;
        } else {
            dp[1] = 1;
        }
        for i in 2..=s.len() {
            let num = s[i-2..i].parse::<u32>().unwrap();
            if num == 0 {
                dp[i] = 0;
            } else {
                if num < 10 || num > 26 {
                    if num % 10 == 0 {
                        dp[i] = 0;
                    } else {
                        // 题目应该说明 01 是不可以使用 A 来表示
                        dp[i]  = dp[i-1];
                    }
                } else{
                    if num % 10 == 0{
                        dp[i] = dp[i-2];
                    } else {
                        dp[i] = dp[i-1] + dp[i-2];
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
    fn num_decodings_test() {
        assert_eq!(Solution::num_decodings("12".to_owned()), 2);
        assert_eq!(Solution::num_decodings("27".to_owned()), 1);
        assert_eq!(Solution::num_decodings("226".to_owned()), 3);
        assert_eq!(Solution::num_decodings("220".to_owned()), 1);
        assert_eq!(Solution::num_decodings("2206".to_owned()), 1);

        assert_eq!(Solution::num_decodings("22006".to_owned()), 0);
        assert_eq!(Solution::num_decodings("2260".to_owned()), 0);
        assert_eq!(Solution::num_decodings("0".to_owned()), 0);
        assert_eq!(Solution::num_decodings("00".to_owned()), 0);

    }
}
