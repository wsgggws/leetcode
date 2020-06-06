// 524. Longest Word in Dictionary through Deleting
// Medium

// Given a string and a string dictionary, find the longest string in the dictionary that can be formed by deleting some characters of the given string. If there are more than one possible results, return the longest word with the smallest lexicographical order. If there is no possible result, return the empty string.

// Example 1:
// Input:
// s = "abpcplea", d = ["ale","apple","monkey","plea"]

// Output:
// "apple"
// Example 2:
// Input:
// s = "abpcplea", d = ["a","b","c"]

// Output:
// "a"
// Note:
// All the strings in the input will only contain lower-case letters.
// The size of the dictionary won't exceed 1,000.
// The length of all the strings in the input won't exceed 1,000.

pub struct Solution {}

use std::cmp::Ordering;
impl Solution {
    pub fn find_longest_word(s: String, d: Vec<String>) -> String {
        let mut result: String = "".to_owned();
        for target in d.iter() {
            // 如果长度小于已经匹配的字符串, 或者已经匹配的字符串字母序小, 都不需要进行判定
            // cmp参考文档进行使用, 如: assert_eq!(10.cmp(&5), Ordering::Greater);
            if target.len() < result.len()
                || (target.len() == result.len() && result.cmp(target) == Ordering::Less)
            {
                continue;
            }
            if Solution::is_substr(&s, target) {
                result = target.clone();
            }
        }
        result
    }
    fn is_substr(s: &String, target: &String) -> bool {
        let (mut i, mut j) = (0, 0);
        let s_chars: Vec<char> = s.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();

        while i < s.len() && j < target.len() {
            if s_chars[i as usize] == target_chars[j as usize] {
                j += 1;
            }
            i += 1;
        }
        return j == target.len();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_longest_word_test() {
        assert_eq!(
            Solution::find_longest_word(
                "abpcplea".to_owned(),
                vec![
                    "ale".to_owned(),
                    "apple".to_owned(),
                    "monkey".to_owned(),
                    "plea".to_owned()
                ]
            ),
            "apple".to_owned()
        );
        // 有字母序的测试数据
        assert_eq!(
            Solution::find_longest_word(
                "abpcplea".to_owned(),
                vec![
                    "ale".to_owned(),
                    "apple".to_owned(),
                    "appla".to_owned(),
                    "monkey".to_owned(),
                    "plea".to_owned()
                ]
            ),
            "appla".to_owned()
        );
    }
}
