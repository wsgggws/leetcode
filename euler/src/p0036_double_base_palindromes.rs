// Double-base palindromes
// Problem 36
// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.

// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

// (Please note that the palindromic number, in either base, may not include leading zeros.)

pub struct Solution {}

impl Solution {
    pub fn double_base_palindromes(n: u64) -> u64 {
        (0..n)
            .filter(|&x| Solution::is_double_base_palindrome(x))
            .sum()
    }

    fn is_double_base_palindrome(x: u64) -> bool {
        let base_2_x = format!("{:b}", x);
        x.to_string().chars().eq(x.to_string().chars().rev()) && base_2_x.chars().eq(base_2_x.chars().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_base_palindromes_test() {
        assert_eq!(Solution::double_base_palindromes(1000000), 872187);
    }

    #[test]
    fn is_double_base_palindrome_test() {
        assert_eq!(Solution::is_double_base_palindrome(585), true);
        assert_eq!(Solution::is_double_base_palindrome(586), false);
    }
}
