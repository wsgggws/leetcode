// 856. Score of Parentheses
// Medium

// Given a balanced parentheses string S, compute the score of the string based on the following rule:

// () has score 1
// AB has score A + B, where A and B are balanced parentheses strings.
// (A) has score 2 * A, where A is a balanced parentheses string.
 

// Example 1:

// Input: "()"
// Output: 1
// Example 2:

// Input: "(())"
// Output: 2
// Example 3:

// Input: "()()"
// Output: 2
// Example 4:

// Input: "(()(()))"
// Output: 6
 

// Note:

// S is a balanced parentheses string, containing only ( and ).
// 2 <= S.length <= 50

pub struct Solution {}

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut result = 0i32;
        let mut stack: Vec<i32> = vec![];
        for c in s.chars() {
            if c == '(' {
                stack.push(result);
                result = 0;
            } else {
                result = stack[stack.len() - 1] + i32::max(result * 2, 1);
                stack.pop();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_of_parentheses_test() {
        assert_eq!(Solution::score_of_parentheses("()".to_owned()), 1);
        assert_eq!(Solution::score_of_parentheses("(())".to_owned()), 2);
        assert_eq!(Solution::score_of_parentheses("()()".to_owned()), 2);
        assert_eq!(Solution::score_of_parentheses("(()(()))".to_owned()), 6);
    }
}
