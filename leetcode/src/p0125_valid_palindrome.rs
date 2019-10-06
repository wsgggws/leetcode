// 125. Valid Palindrome
// Easy

// Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.

// Note: For the purpose of this problem, we define empty string as valid palindrome.

// Example 1:

// Input: "A man, a plan, a canal: Panama"
// Output: true
// Example 2:

// Input: "race a car"
// Output: false

pub struct Solution {}


impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() { return  true }
        let a_str: Vec<char> = s.chars().collect();
        let (mut i, mut j) = (0_usize, a_str.len() - 1);
        while i < j {
            while i < a_str.len() && !a_str[i].is_ascii_alphanumeric(){ i += 1; }
            while j > 0 && !a_str[j].is_ascii_alphanumeric() { j -= 1; }
            if i >= j { break }
            if a_str[i].to_ascii_lowercase() != a_str[j].to_ascii_lowercase() {
                return false
            }
            i += 1; j -= 1;
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert_eq!(Solution::is_palindrome("".to_string()), true);
        assert_eq!(Solution::is_palindrome("t".to_string()), true);
        assert_eq!(Solution::is_palindrome("tiat".to_string()), false);
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }
}
