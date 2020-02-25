// 5. Longest Palindromic Substring
// Medium

// Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.

// Example 1:

// Input: "babad"
// Output: "bab"
// Note: "aba" is also a valid answer.
// Example 2:

// Input: "cbbd"
// Output: "bb"

pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // 暴力破解, Time Limit Exceeded(The last test input)
        let lens = s.len();
        if lens < 2 {
            return s.clone();
        }
        let mut max_len = 0usize;
        let (mut index_i, mut index_j) = (0usize, 0usize);
        for i in 0..lens {
            for j in i..lens {
                let cur_len = j - i + 1usize;
                if cur_len > max_len && Solution::is_palindrome(&s, i, j) {
                    max_len = cur_len;
                    index_i = i;
                    index_j = j;
                }
            }
        }
        s[index_i..=index_j].to_owned()
    }
    fn is_palindrome(s: &String, i: usize, j: usize) -> bool {
        let chars: Vec<char> = s[i..=j].chars().collect();
        let (mut x, mut y) = (0i32, (j - i) as i32);
        while x < y {
            if chars[x as usize] != chars[y as usize] {
                return false;
            }
            x += 1;
            y -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindrome_test() {
        assert_eq!(Solution::longest_palindrome("".to_owned()), "".to_owned());
        assert_eq!(Solution::longest_palindrome("a".to_owned()), "a".to_owned());
        assert_eq!(Solution::longest_palindrome("abcdef".to_owned()), "a".to_owned());
        assert_eq!(Solution::longest_palindrome("babad".to_owned()), "bab".to_owned());
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb".to_owned());
        assert_eq!(Solution::longest_palindrome("nypdmqqgauepeyfvwcdpbmmaxfwxmmtswfuwldtvqcisywalfnvovuordczxlyzqmslxilpnenbuwbcpebneovitwkkswsijajnkwkfbxnulmwotgrmpklntfyjavccbrgwqynryeoswmhsqzcwnudkuvfkikjxjkjpghsytjfkpvkjpvblamdeegeohospporbtorkbuggbawgodhxpscfksjbirxvjyjapwwushmnqsxktnslvonlwvuseinrmwvfqjgzpkwcqfzfdbbmcngmsoeegudwjvldqmaomwbqvijesnpxiqvtfeiqebrfjhtvjdwkopyfzaslewdjnkmalvfinbuouwcgnfecjtdzwycxrynxepbcsroyzrsgiiuaszvatwyuxinwhni".to_owned()), "kjxjk".to_owned());
}
}
