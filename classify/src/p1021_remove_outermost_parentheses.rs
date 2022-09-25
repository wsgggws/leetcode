// 1021. Remove Outermost Parentheses
// Easy

// A valid parentheses string is either empty (""), "(" + A + ")", or A + B, where A and B are valid parentheses strings, and + represents string concatenation.  For example, "", "()", "(())()", and "(()(()))" are all valid parentheses strings.

// A valid parentheses string S is primitive if it is nonempty, and there does not exist a way to split it into S = A+B, with A and B nonempty valid parentheses strings.

// Given a valid parentheses string S, consider its primitive decomposition: S = P_1 + P_2 + ... + P_k, where P_i are primitive valid parentheses strings.

// Return S after removing the outermost parentheses of every primitive string in the primitive decomposition of S.



// Example 1:

// Input: "(()())(())"
// Output: "()()()"
// Explanation:
// The input string is "(()())(())", with primitive decomposition "(()())" + "(())".
// After removing outer parentheses of each part, this is "()()" + "()" = "()()()".
// Example 2:

// Input: "(()())(())(()(()))"
// Output: "()()()()(())"
// Explanation:
// The input string is "(()())(())(()(()))", with primitive decomposition "(()())" + "(())" + "(()(()))".
// After removing outer parentheses of each part, this is "()()" + "()" + "()(())" = "()()()()(())".
// Example 3:

// Input: "()()"
// Output: ""
// Explanation:
// The input string is "()()", with primitive decomposition "()" + "()".
// After removing outer parentheses of each part, this is "" + "" = "".


// Note:

// S.length <= 10000
// S[i] is "(" or ")"
// S is a valid parentheses string

pub struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        // 题目要求去掉外层的括号
        let mut result: Vec<char> = vec![];
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if c == '(' && stack.len() == 0 {
                stack.push(c);
            } else if c == '('  && stack.len() != 0 {
                stack.push(c);
                result.push(c);
            } else if c == ')' && stack.len() == 1 {
                stack.pop();
            } else if c == ')' && stack.len() > 1 {
                stack.pop();
                result.push(c);
            }
        }
        if result.len() == 0 {
            "".to_owned()
        } else {
            result.iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_outer_parentheses_test() {
        assert_eq!(Solution::remove_outer_parentheses("(()())(())".to_owned()), "()()()".to_owned());
        assert_eq!(Solution::remove_outer_parentheses("(()())(())(()(()))".to_owned()), "()()()()(())".to_owned());
        assert_eq!(Solution::remove_outer_parentheses("()()".to_owned()), "".to_owned());
    }
}
