// 20. Valid Parentheses
// Easy

// Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Note that an empty string is also considered valid.

// Example 1:

// Input: "()"
// Output: true
// Example 2:

// Input: "()[]{}"
// Output: true
// Example 3:

// Input: "(]"
// Output: false
// Example 4:

// Input: "([)]"
// Output: false
// Example 5:

// Input: "{[]}"
// Output: true

pub struct Solution {}


impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for ch in s.chars(){
            match ch {
                '(' | '{' | '[' => stack.push(ch),
                ')' => {
                    let c = stack.pop();
                    match c {
                        Some('(') => {},
                        _ => { return false; },
                    }
                },
                '}' => { 
                    let c = stack.pop();
                    match c {
                        Some('{') => {},
                        _ => { return false; }
                    }
                },
                ']' => { 
                    let c = stack.pop(); 
                    match c {
                        Some('[') => {},
                        _ => { return false; }
                    }
                },
                _ => panic!("it's no possible"),
            }
        }
        if stack.len() > 0 { return false; }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_test() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("({})".to_string()), true);
        assert_eq!(Solution::is_valid("".to_string()), true);
    }
}
