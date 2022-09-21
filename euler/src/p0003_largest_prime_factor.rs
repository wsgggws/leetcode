// Largest prime factor
// Problem 3
// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

pub struct Solution {}

impl Solution {
    pub fn largest_prime_factor(num: u64) -> u64 {
        let mut candidates = 2u64..;
        let mut n = num;
        let mut result = 2u64;
        while n > 1u64 {
            let x = candidates.next().unwrap();
            while n % x == 0 {
                n /= x;
            }
            result = x;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_prime_factor_test() {
        assert_eq!(Solution::largest_prime_factor(2), 2);
        assert_eq!(Solution::largest_prime_factor(13195), 29);
        assert_eq!(Solution::largest_prime_factor(600851475143), 6857);
    }
}
