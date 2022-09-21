// 1513. Number of Substrings With Only 1s
// Medium

// Given a binary string s (a string consisting only of '0' and '1's).

// Return the number of substrings with all characters 1's.

// Since the answer may be too large, return it modulo 10^9 + 7.

 

// Example 1:

// Input: s = "0110111"
// Output: 9
// Explanation: There are 9 substring in total with only 1's characters.
// "1" -> 5 times.
// "11" -> 3 times.
// "111" -> 1 time.
// Example 2:

// Input: s = "101"
// Output: 2
// Explanation: Substring "1" is shown 2 times in s.
// Example 3:

// Input: s = "111111"
// Output: 21
// Explanation: Each substring contains only 1's characters.
// Example 4:

// Input: s = "000"
// Output: 0
 

// Constraints:

// s[i] == '0' or s[i] == '1'
// 1 <= s.length <= 10^5

pub struct Solution {}

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut dp: Vec<i64> = vec![0; s.len()];
        let chs: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            if i == 0 {
                dp[i] = if chs[i] == '1' { 1 } else { 0 };
            } else if chs[i-1] == '1' && chs[i] == '1'{
                dp[i] = (dp[i-1] + 1) % 1000000007
            } else if chs[i] == '1' {
                dp[i] = 1;
            }
        }
        dp.iter().fold(0, |result, x| (result + x) % 1000000007 ) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_sub_test() {
        assert_eq!(Solution::num_sub("000".to_owned()), 0);
        assert_eq!(Solution::num_sub("0110111".to_owned()), 9);
        assert_eq!(Solution::num_sub("111111".to_owned()), 21);
    }
}
