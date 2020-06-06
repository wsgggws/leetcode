// 58. Length of Last Word
// Easy

// Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word in the string.

// If the last word does not exist, return 0.

// Note: A word is defined as a character sequence consists of non-space characters only.

// Example:

// Input: "Hello World"
// Output: 5

pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let splits: Vec<&str> = s.trim().split(" ").collect();
        let len = splits.len();
        let last_word = splits[len - 1].trim();
        last_word.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_last_word_test() {
        // 注意该测试用例,表示只要有字母, 就会有长度
        assert_eq!(Solution::length_of_last_word("a ".to_string()), 1);
        assert_eq!(Solution::length_of_last_word("Hello World ".to_string()), 5);
        assert_eq!(Solution::length_of_last_word("".to_string()), 0);
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }
}
