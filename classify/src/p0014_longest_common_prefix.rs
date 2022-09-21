// 14. Longest Common Prefix
// Easy

// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:

// Input: ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
// Note:

// All given inputs are in lowercase letters a-z.

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        if strs.len() == 1 {
            return strs[0].to_string();
        }
        let mut index = 0;
        let mut flag = false;
        for _ in 0..strs[0].len() {
            for a_str in &strs[1..] {
                if (index >= a_str.len()) || (strs[0][index..index + 1] != a_str[index..index + 1])
                {
                    flag = true;
                    break;
                }
            }
            if flag {
                break;
            } else {
                index += 1;
            }
        }
        if index == 0 {
            return "".to_string();
        }
        strs[0][..index].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_prefix_test() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "".to_string());
        assert_eq!(
            Solution::longest_common_prefix(vec!["abc".to_string()]),
            "abc".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
}
