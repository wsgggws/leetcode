// 394. Decode String
// Medium

// Given an encoded string, return its decoded string.

// The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.

// You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.

// Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].



// Example 1:

// Input: s = "3[a]2[bc]"
// Output: "aaabcbc"
// Example 2:

// Input: s = "3[a2[c]]"
// Output: "accaccacc"
// Example 3:

// Input: s = "2[abc]3[cd]ef"
// Output: "abcabccdcdcdef"
// Example 4:

// Input: s = "abc3[cd]xyz"
// Output: "abccdcdcdxyz"

pub struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if c == ']' {

                // 取[]里面的所有字符
                let mut tmp: Vec<char> = vec![];
                while let Some(ch) = stack.pop() {
                    if ch == '[' {
                        break
                    } else {
                        tmp.push(ch);
                    }
                }

                // 取数字
                let mut nums: Vec<char> = vec![];
                while let Some(ch) = stack.pop() {
                    if '0' <= ch && ch <= '9' {
                        nums.push(ch);
                    } else {
                        stack.push(ch);
                        break;
                    }
                }

                // 重新推入栈
                let number = nums.iter().rev().collect::<String>().parse::<i32>().unwrap();
                for _ in 0..number {
                    for t_c in tmp.iter().rev() {
                        stack.push(*t_c);
                    }
                }
            } else {
                stack.push(c)
            }
        }
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_string_test() {
        assert_eq!(Solution::decode_string("3[a]2[bc]".to_owned()), "aaabcbc".to_owned());
        assert_eq!(Solution::decode_string("3[a2[c]]".to_owned()), "accaccacc".to_owned());
        assert_eq!(Solution::decode_string("2[abc]3[cd]ef".to_owned()), "abcabccdcdcdef".to_owned());
        assert_eq!(Solution::decode_string("abc3[cd]xyz".to_owned()), "abccdcdcdxyz".to_owned());
        assert_eq!(Solution::decode_string("10[le]".to_owned()), "lelelelelelelelelele".to_owned());
    }
}
