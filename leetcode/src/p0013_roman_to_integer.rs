// 13. Roman to Integer
// Easy

// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9. 
// X can be placed before L (50) and C (100) to make 40 and 90. 
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.

// Example 1:

// Input: "III"
// Output: 3
// Example 2:

// Input: "IV"
// Output: 4
// Example 3:

// Input: "IX"
// Output: 9
// Example 4:

// Input: "LVIII"
// Output: 58
// Explanation: L = 50, V= 5, III = 3.
// Example 5:

// Input: "MCMXCIV"
// Output: 1994
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.

pub struct Solution {}


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let symbols = vec![
            ("MMM", 3, 3000),
            ("MM", 2, 2000),
            ("M", 1, 1000),
            ("CM", 2, 900),
            ("DCCC", 4, 800),
            ("DCC", 3, 700),
            ("DC", 2, 600),
            ("D", 1, 500),
            ("CD", 2, 400),
            ("CCC", 3, 300),
            ("CC", 2, 200),
            ("C", 1, 100),
            ("XC", 2, 90),
            ("LXXX", 4, 80),
            ("LXX", 3, 70),
            ("LX", 2, 60),
            ("L", 1, 50),
            ("XL", 2, 40),
            ("XXX", 3, 30),
            ("XX", 2, 20),
            ("X", 1, 10),
            ("IX", 2, 9),
            ("VIII", 4, 8),
            ("VII", 3, 7),
            ("VI", 2, 6),
            ("V", 1, 5),
            ("IV", 2, 4),
            ("III", 3, 3),
            ("II", 2, 2),
            ("I", 1, 1),
        ];
        let mut result = 0;
        let mut index = 0;
        for symbol in &symbols {
            if index+symbol.1 <= s.len() && symbol.0 == &s[index..(index+symbol.1)] {
                index += symbol.1;
                result += symbol.2;
                if index >= s.len() { break; }
            }
        }
        result
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn roman_to_int_test() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
