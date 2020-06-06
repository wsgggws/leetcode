// 680. Valid Palindrome II
// Given a non-empty string s, you may delete at most one character. Judge whether you can make it a palindrome.

// Example 1:
// Input: "aba"
// Output: True
// Example 2:
// Input: "abca"
// Output: True
// Explanation: You could delete the character 'c'.
// Note:
// The string will only contain lowercase characters a-z. The maximum length of the string is 50000.

pub struct Solution {}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let s_chars: Vec<char> = s.chars().collect();
        let (mut i, mut j) = (0_usize, s_chars.len() - 1_usize);
        while i <= j {
            if s_chars[i] != s_chars[j] {
                return Solution::is_palindrome_string(&s_chars, i + 1, j)
                    || Solution::is_palindrome_string(&s_chars, i, j - 1);
            }
            i += 1;
            j -= 1;
        }
        true
    }
    #[allow(dead_code)]
    // 这里使用&进行借用
    fn is_palindrome_string(s_chars: &Vec<char>, i: usize, j: usize) -> bool {
        let (mut i, mut j) = (i, j);
        while i <= j {
            if s_chars[i] != s_chars[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn valid_palindrome_test() {
        assert_eq!(Solution::valid_palindrome("".to_owned()), true);
        assert_eq!(Solution::valid_palindrome("aba".to_owned()), true);
        assert_eq!(Solution::valid_palindrome("abca".to_owned()), true);
        assert_eq!(Solution::valid_palindrome("abcda".to_owned()), false);
    }
}
