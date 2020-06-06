// 8. String to Integer (atoi)
// Medium

// Implement atoi which converts a string to an integer.

// The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.

// The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.

// If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.

// If no valid conversion could be performed, a zero value is returned.

// Note:

// Only the space character ' ' is considered as whitespace character.
// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. If the numerical value is out of the range of representable values, INT_MAX (231 − 1) or INT_MIN (−231) is returned.
// Example 1:

// Input: "42"
// Output: 42
// Example 2:

// Input: "   -42"
// Output: -42
// Explanation: The first non-whitespace character is '-', which is the minus sign.
//              Then take as many numerical digits as possible, which gets 42.
// Example 3:

// Input: "4193 with words"
// Output: 4193
// Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
// Example 4:

// Input: "words and 987"
// Output: 0
// Explanation: The first non-whitespace character is 'w', which is not a numerical
//              digit or a +/- sign. Therefore no valid conversion could be performed.
// Example 5:

// Input: "-91283472332"
// Output: -2147483648
// Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
//              Thefore INT_MIN (−231) is returned.

pub struct Solution {}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let a_str: String = str.to_string();
        let a_str = a_str.trim();
        if a_str.len() == 0 {
            return 0;
        }
        let mut flag = false;
        let mut result = String::new();
        for (index, c) in a_str.chars().enumerate() {
            if index == 0 && c == '-' {
                flag = true;
            }
            // need jupge the plus
            else if index == 0 && c == '+' {
                flag = false;
            } else if '0' <= c && c <= '9' {
                if '0' == c && result.len() == 0 {
                    // do nothing, if the all first char is '0'
                } else {
                    result.push(c);
                }
            } else {
                break;
            }
        }
        // empty string
        if result.len() == 0 {
            return 0;
        }

        let int_min = (-2_i64.pow(31)) as i32;
        let int_max = (2_i64.pow(31) - 1) as i32;
        // string len > 10, int_min or int_max
        if result.len() >= 11 {
            if flag {
                return int_min;
            } else {
                return int_max;
            }
        }

        let a: i64 = result.parse().expect("error.");
        if !flag && a >= 2_i64.pow(31) {
            return int_max;
        }
        if flag && a > 2_i64.pow(31) {
            return int_min;
        }
        if flag {
            return (0_i64 - a) as i32;
        }
        a as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_atoi_test() {
        assert_eq!(Solution::my_atoi("".to_string()), 0);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("    -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("+1".to_string()), 1);
        assert_eq!(Solution::my_atoi("+42".to_string()), 42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("-123 a".to_string()), -123);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("2147483648".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi("-2147483648".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("2147483647".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi("-2147483649".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("21474aaa".to_string()), 21474);
        assert_eq!(
            Solution::my_atoi("  0000000000012345678".to_string()),
            12345678
        );
        assert_eq!(Solution::my_atoi("  1001".to_string()), 1001);
        assert_eq!(Solution::my_atoi("  -1001".to_string()), -1001);
    }
}
