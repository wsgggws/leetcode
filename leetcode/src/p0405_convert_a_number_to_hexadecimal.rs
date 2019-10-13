// 405. Convert a Number to Hexadecimal
// Easy

// Given an integer, write an algorithm to convert it to hexadecimal. For negative integer, two’s complement method is used.

// Note:

// All letters in hexadecimal (a-f) must be in lowercase.
// The hexadecimal string must not contain extra leading 0s. If the number is zero, it is represented by a single zero character '0'; otherwise, the first character in the hexadecimal string will not be the zero character.
// The given number is guaranteed to fit within the range of a 32-bit signed integer.
// You must not use any method provided by the library which converts/formats the number to hex directly.
// Example 1:

// Input:
// 26

// Output:
// "1a"
// Example 2:

// Input:
// -1

// Output:
// "ffffffff"

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 { return "0".to_owned(); }
        let binary = format!("{:032b}", num);

        let mut map = HashMap::with_capacity(16);
        map.insert("0000", '0');
        map.insert("0001", '1');
        map.insert("0010", '2');
        map.insert("0011", '3');
        map.insert("0100", '4');
        map.insert("0101", '5');
        map.insert("0110", '6');
        map.insert("0111", '7');
        map.insert("1000", '8');
        map.insert("1001", '9');
        map.insert("1010", 'a');
        map.insert("1011", 'b');
        map.insert("1100", 'c');
        map.insert("1101", 'd');
        map.insert("1110", 'e');
        map.insert("1111", 'f');

        let mut result = String::new();
        let mut i = 0;
        for _ in 0..8 {
            if let Some(&ch) = map.get(&binary[i..i+4]) {
                result.push(ch);
            }
            i += 4;
        }
        result.trim_start_matches('0').to_owned()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hex_test() {
        assert_eq!(Solution::to_hex(26), "1a".to_owned());
        assert_eq!(Solution::to_hex(-1), "ffffffff".to_owned());
    }
}
