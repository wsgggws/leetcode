// 1047. Remove All Adjacent Duplicates In String
// Easy

// Given a string S of lowercase letters, a duplicate removal consists of choosing two adjacent and equal letters, and removing them.

// We repeatedly make duplicate removals on S until we no longer can.

// Return the final string after all such duplicate removals have been made.  It is guaranteed the answer is unique.

 

// Example 1:

// Input: "abbaca"
// Output: "ca"
// Explanation: 
// For example, in "abbaca" we could remove "bb" since the letters are adjacent and equal, and this is the only possible move.  The result of this move is that the string is "aaca", of which only "aa" is possible, so the final string is "ca".
 

// Note:

// 1 <= S.length <= 20000
// S consists only of English lowercase letters.

pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if stack.len() > 0 && stack[stack.len() -1] == c {
                stack.pop();
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
    fn remove_duplicates_test() {
        assert_eq!(Solution::remove_duplicates("abbaca".to_owned()), "ca".to_owned());
    }
}
