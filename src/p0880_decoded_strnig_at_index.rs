// 880. Decoded String at Index
// Medium

// An encoded string S is given.  To find and write the decoded string to a tape, the encoded string is read one character at a time and the following steps are taken:

// If the character read is a letter, that letter is written onto the tape.
// If the character read is a digit (say d), the entire current tape is repeatedly written d-1 more times in total.
// Now for some encoded string S, and an index K, find and return the K-th letter (1 indexed) in the decoded string.


// Example 1:

// Input: S = "leet2code3", K = 10
// Output: "o"
// Explanation: 
// The decoded string is "leetleetcodeleetleetcodeleetleetcode".
// The 10th letter in the string is "o".
// Example 2:

// Input: S = "ha22", K = 5
// Output: "h"
// Explanation: 
// The decoded string is "hahahaha".  The 5th letter is "h".
// Example 3:

// Input: S = "a2345678999999999999999", K = 1
// Output: "a"
// Explanation: 
// The decoded string is "a" repeated 8301530446056247680 times.  The 1st letter is "a".
 

// Note:

// 2 <= S.length <= 100
// S will only contain lowercase letters and digits 2 through 9.
// S starts with a letter.
// 1 <= K <= 10^9
// The decoded string is guaranteed to have less than 2^63 letters.

pub struct Solution {}

// NOTE: Time Limit Exceeded !!!
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if '2' <= c && c <= '9' {
                let copied_stack = stack.clone();
                for _ in 1..c.to_digit(10).unwrap() {
                    stack.extend(copied_stack.clone());
                    if stack.len() >= k as usize {
                        return stack[(k - 1) as usize].to_string();
                    }
                }
            } else {
                stack.push(c);
            }
        }
        stack[(k - 1) as usize].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_at_index_test() {
        assert_eq!(
            Solution::decode_at_index("leet2code3".to_owned(), 10),
            "o".to_owned()
        );
        assert_eq!(
            Solution::decode_at_index("ha22".to_owned(), 5),
            "h".to_owned()
        );
        assert_eq!(
            Solution::decode_at_index("a23456789999999999".to_owned(), 1),
            "a".to_owned()
        );
        // NOTE: Time Limit Exceeded !!!
        assert_eq!(
            Solution::decode_at_index("y959q969u3hb22odq595".to_owned(), 222280369),
            "y".to_owned()
        );
    }
}
