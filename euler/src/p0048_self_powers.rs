// Self powers
// Problem 48
// The series, 11 + 22 + 33 + ... + 1010 = 10405071317.

// Find the last ten digits of the series, 11 + 22 + 33 + ... + 10001000.

pub struct Solution {}

impl Solution {
    pub fn self_powers(n: u64) -> u64 {
        (1..=n)
            .map(|x| Solution::get_last_ten_digits(x))
            .sum::<u64>() % 10_000_000_000
    }

    fn get_last_ten_digits(num: u64) -> u64 {
        (1..num)
            .fold(num, |result, _| result * num % 10_000_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_powers_test() {
        assert_eq!(Solution::self_powers(10), 0405071317);
        assert_eq!(Solution::self_powers(100), 9027641920);
        assert_eq!(Solution::self_powers(1000), 9110846700);
    }

    #[test]
    fn get_last_ten_digits_test() {
        assert_eq!(Solution::get_last_ten_digits(1), 1);
        assert_eq!(Solution::get_last_ten_digits(2), 4);
        assert_eq!(Solution::get_last_ten_digits(3), 27);
        assert_eq!(Solution::get_last_ten_digits(10), 0);
    }
}
