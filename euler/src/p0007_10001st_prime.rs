// 10001st prime
// Problem 7
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// What is the 10 001st prime number?

pub struct Solution {}

impl Solution {
    pub fn get_st_prime(st_value: usize) -> u64 {
        // nth(0) returns the first value, nth(1) the second
        (2u64..)
            .filter(|&num| Solution::is_prime(num))
            .nth(st_value-1usize)
            .unwrap()
    }

    fn is_prime(num: u64) -> bool {
        if num == 2 || num == 3 {
            return true;
        }
        !(2..=(num as f64).sqrt().ceil() as u64)
            .any(|value| num % value == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_st_prime_test() {
        assert_eq!(Solution::get_st_prime(1), 2);
        assert_eq!(Solution::get_st_prime(2), 3);
        assert_eq!(Solution::get_st_prime(6), 13);
        assert_eq!(Solution::get_st_prime(7), 17);
        assert_eq!(Solution::get_st_prime(8), 19);
        assert_eq!(Solution::get_st_prime(10001), 104743);
    }
}
