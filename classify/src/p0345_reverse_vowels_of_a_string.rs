// 345. Reverse Vowels of a String
// Write a function that takes a string as input and reverse only the vowels of a string.

// Example 1:

// Input: "hello"
// Output: "holle"
// Example 2:

// Input: "leetcode"
// Output: "leotcede"
// Note:
// The vowels does not include the letter "y".

pub struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let mut chars: Vec<char> = s.chars().collect();
        let mut indexes: Vec<usize> = Vec::new();

        for (index, ch) in chars.iter().enumerate() {
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => indexes.push(index),
                _ => {}
            }
        }

        if indexes.len() == 0 || indexes.len() == 1 {
            return s;
        }

        let mut i = 0_usize;
        let mut j = indexes.len() - 1_usize;
        while i < j {
            chars.swap(indexes[i], indexes[j]);
            i += 1;
            j -= 1;
        }
        chars.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_vowels_test() {
        assert_eq!(Solution::reverse_vowels("".to_string()), "".to_string());
        assert_eq!(Solution::reverse_vowels("a".to_string()), "a".to_string());
        assert_eq!(Solution::reverse_vowels("aa".to_string()), "aa".to_string());
        assert_eq!(
            Solution::reverse_vowels("aio".to_string()),
            "oia".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("hello".to_string()),
            "holle".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("hEllo".to_string()),
            "hollE".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("   ".to_string()),
            "   ".to_string()
        );
    }
}
