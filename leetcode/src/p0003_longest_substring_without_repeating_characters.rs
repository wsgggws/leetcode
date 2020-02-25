// 3. Longest Substring Without Repeating Characters
// Medium

// Given a string, find the length of the longest substring without repeating characters.

// Example 1:

// Input: "abcabcbb"
// Output: 3 
// Explanation: The answer is "abc", with the length of 3. 
// Example 2:

// Input: "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3. 
//              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let lens = s.len();
        if lens < 2 {
            return lens as i32;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut counts: Vec<usize> = vec![0; 256];
        let mut result = 0;
        let mut j = 0usize;
        for i in 0..lens {
            counts[chars[i] as usize] += 1;
            if counts[chars[i] as usize] > 1 {
                while counts[chars[i] as usize] > 1 {
                    counts[chars[j] as usize] -= 1;
                    j += 1;
                }
            }
            result = i32::max(result, (i - j + 1) as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_longest_substring_test() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_owned()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_owned()), 3);
    }
}
