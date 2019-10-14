// 443. String Compression
// Easy

// Given an array of characters, compress it in-place.

// The length after compression must always be smaller than or equal to the original array.

// Every element of the array should be a character (not int) of length 1.

// After you are done modifying the input array in-place, return the new length of the array.

 
// Follow up:
// Could you solve it using only O(1) extra space?

 
// Example 1:

// Input:
// ["a","a","b","b","c","c","c"]

// Output:
// Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]

// Explanation:
// "aa" is replaced by "a2". "bb" is replaced by "b2". "ccc" is replaced by "c3".
 

// Example 2:

// Input:
// ["a"]

// Output:
// Return 1, and the first 1 characters of the input array should be: ["a"]

// Explanation:
// Nothing is replaced.
 

// Example 3:

// Input:
// ["a","b","b","b","b","b","b","b","b","b","b","b","b"]

// Output:
// Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].

// Explanation:
// Since the character "a" does not repeat, it is not compressed. "bbbbbbbbbbbb" is replaced by "b12".
// Notice each digit has it's own entry in the array.
 

// Note:

// All characters have an ASCII value in [35, 126].
// 1 <= len(chars) <= 1000.

pub struct Solution {}


impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut first_char: char = chars[0];
        let mut part_len = 0;
        let lens = chars.len();
        for _ in 0..lens {
            let temp: char = chars.remove(0);
            if first_char == temp { part_len += 1; }
            else {
                chars.push(first_char);
                if part_len > 1 {
                    chars.extend(part_len.to_string().chars());
                }
                first_char = temp;
                part_len = 1;
            }
        }
        chars.push(first_char);
        if part_len > 1 {
            chars.extend(part_len.to_string().chars());
        }
        chars.len() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_test() {
        let mut test_vec = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut test_vec), 6);
        assert_eq!(test_vec, vec!['a', '2', 'b', '2', 'c', '3']);

        let mut test_vec = vec!['a'];
        assert_eq!(Solution::compress(&mut test_vec), 1);
        assert_eq!(test_vec, vec!['a']);

        let mut test_vec = vec!['a','b','b','b','b','b','b','b','b','b','b','b','b'];
        assert_eq!(Solution::compress(&mut test_vec), 4);
        assert_eq!(test_vec, vec!['a', 'b', '1', '2']);
    }
}
