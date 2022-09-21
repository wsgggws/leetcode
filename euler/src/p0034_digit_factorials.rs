// Digit factorials
// Problem 34
// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

// Find the sum of all numbers which are equal to the sum of the factorial of their digits.

// Note: as 1! = 1 and 2! = 2 are not sums they are not included.

pub struct Solution {}

impl Solution {
    pub fn digit_factorials() -> u64 {
        (10..1000000)
            .filter(|&x| Solution::is_digit_factorial(x))
            .sum()
    }

    fn is_digit_factorial(x: u64) -> bool {
        // 0~9的阶乘
        let factorials: Vec<u64> = vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
        let mut num = x;
        let mut sum = 0;
        while num > 0 {
            sum += factorials[(num % 10) as usize];
            num /= 10;
        }
        sum == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_factorials_test() {
        assert_eq!(Solution::digit_factorials(), 40730);
    }

    #[test]
    fn is_digit_factorial_test() {
        assert_eq!(Solution::is_digit_factorial(144), false);
        assert_eq!(Solution::is_digit_factorial(145), true);
        assert_eq!(Solution::is_digit_factorial(40585), true);
    }
}
