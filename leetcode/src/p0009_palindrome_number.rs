// 9. Palindrome Number
// Easy

// Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.

// Example 1:

// Input: 121
// Output: true
// Example 2:

// Input: -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
// Example 3:

// Input: 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
// Follow up:

// Coud you solve it without converting the integer to a string?

pub struct Solution {}


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 && x <= 9 { return true}
        if x < 0 || x % 10 == 0 { return false}
        let mut digits: Vec<i32> = Vec::new();
        let mut input = x;
        while input != 0 {
            digits.push(input % 10);
            input /= 10;
        }
        let len = digits.len();
        let mut index = 0;
        while index < len/2 {
            if digits[index] != digits[len-index-1] {return false}
            index += 1;
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(9), true);
        assert_eq!(Solution::is_palindrome(-32), false);
        assert_eq!(Solution::is_palindrome(100), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(1321), false);
        assert_eq!(Solution::is_palindrome(2222), true);
        assert_eq!(Solution::is_palindrome(11222211), true);
    }
}
