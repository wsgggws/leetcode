// 171. Excel Sheet Column Number
// Easy

// Given a column title as appear in an Excel sheet, return its corresponding column number.

// For example:

//     A -> 1
//     B -> 2
//     C -> 3
//     ...
//     Z -> 26
//     AA -> 27
//     AB -> 28 
//     ...
// Example 1:

// Input: "A"
// Output: 1
// Example 2:

// Input: "AB"
// Output: 28
// Example 3:

// Input: "ZY"
// Output: 701

pub struct Solution {}


impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let chars: Vec<char> = s.chars().rev().collect();
        let mut result = 0;
        let mut base = 1;
        for ch in chars {
            result += (((ch as u8) - ('A' as u8) + 1_u8) as i32) * base;
            base *= 26;
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_to_number_test() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("Z".to_string()), 26);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    }
}

