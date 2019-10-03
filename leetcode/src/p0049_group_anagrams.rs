// 49. Group Anagrams
// Medium

// Given an array of strings, group anagrams together.

// Example:

// Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
// Output:
// [
//   ["ate","eat","tea"],
//   ["nat","tan"],
//   ["bat"]
// ]
// Note:

// All inputs will be in lowercase.
// The order of your output does not matter.

pub struct Solution {}


use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for a_str in strs.into_iter() {
            let mut chars: Vec<char> = a_str.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            map.entry(chars).or_insert(Vec::new()).push(a_str);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    // TODO 每次测试vec列表的输出顺序不一致, 还不知道如何测试这种情况
    fn group_anagrams_test() {
        assert_eq!(
            Solution::group_anagrams(
                vec![
                    "eat".to_string(),
                    "tea".to_string(),
                    "tan".to_string(),
                    "ate".to_string(),
                    "nat".to_string(),
                    "bat".to_string()
                ]
            ),
            vec![
                vec!["tan".to_string(), "nat".to_string()],
                vec!["bat".to_string()],
                vec!["eat".to_string(),"ate".to_string(),"tea".to_string()]
            ]
        );
    }
}
