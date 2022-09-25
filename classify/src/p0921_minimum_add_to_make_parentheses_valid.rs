// 921. Minimum Add to Make Parentheses Valid
// Medium

// Given a string S of '(' and ')' parentheses, we add the minimum number of parentheses ( '(' or ')', and in any positions ) so that the resulting parentheses string is valid.

// Formally, a parentheses string is valid if and only if:

// It is the empty string, or
// It can be written as AB (A concatenated with B), where A and B are valid strings, or
// It can be written as (A), where A is a valid string.
// Given a parentheses string, return the minimum number of parentheses we must add to make the resulting string valid.



// Example 1:

// Input: "())"
// Output: 1
// Example 2:

// Input: "((("
// Output: 3
// Example 3:

// Input: "()"
// Output: 0
// Example 4:

// Input: "()))(("
// Output: 4


// Note:

// S.length <= 1000
// S only consists of '(' and ')' characters.

pub struct Solution {}

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack: Vec<char> = vec![];
        let mut result = 0;
        for c in s.chars() {
            if c == '(' {
                stack.push('(');
            } else {
                if stack.len() == 0 {
                    result += 1;
                } else {
                    stack.pop();
                }
            }
        }
        result + stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_add_to_make_valid_test() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_owned()), 1);
        assert_eq!(Solution::min_add_to_make_valid("(((".to_owned()), 3);
        assert_eq!(Solution::min_add_to_make_valid("()".to_owned()), 0);
        assert_eq!(Solution::min_add_to_make_valid("()))((".to_owned()), 4);
    }
}
