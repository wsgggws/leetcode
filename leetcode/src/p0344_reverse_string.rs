// 344. Reverse String
// Easy

// Write a function that reverses a string. The input string is given as an array of characters char[].

// Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.

// You may assume all the characters consist of printable ascii characters.

// Example 1:

// Input: ["h","e","l","l","o"]
// Output: ["o","l","l","e","h"]
// Example 2:

// Input: ["H","a","n","n","a","h"]
// Output: ["h","a","n","n","a","H"]

pub struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_string_test() {
        let mut arr: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut arr);
        assert_eq!(arr, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
