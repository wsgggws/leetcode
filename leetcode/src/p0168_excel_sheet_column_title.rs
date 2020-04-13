// 168. Excel Sheet Column Title
// Easy

// Given a positive integer, return its corresponding column title as appear in an Excel sheet.

// For example:

//     1 -> A
//     2 -> B
//     3 -> C
//     ...
//     26 -> Z
//     27 -> AA
//     28 -> AB
//     ...
// Example 1:

// Input: 1
// Output: "A"
// Example 2:

// Input: 28
// Output: "AB"
// Example 3:

// Input: 701
// Output: "ZY"

pub struct Solution {}

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let base = 26;
        let mut n = n;
        let mut result = String::new();
        while n > 0 {
            let mut code = (n % base) as u8;
            n = n / base;
            if code == 0 {
                n -= 1;
                code = base as u8;
            };
            let ch = (('A' as u8) + (code - 1_u8)) as char;
            result.insert(0, ch);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_title_test() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
        assert_eq!(Solution::convert_to_title(26), "Z".to_string());
        assert_eq!(Solution::convert_to_title(27), "AA".to_string());
        assert_eq!(Solution::convert_to_title(28), "AB".to_string());
        assert_eq!(Solution::convert_to_title(52), "AZ".to_string());
    }
}
