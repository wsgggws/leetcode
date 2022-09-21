// Summation of primes
// Problem 10
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.

pub struct Solution {}

impl Solution {
    pub fn summation_of_primes(max_value: u64) -> u64 {
        (2u64..)
            .filter(|&num| Solution::is_prime(num))
            .take_while(|&num| num < max_value)
            .sum::<u64>()
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
    fn summation_of_primes_test() {
        assert_eq!(Solution::summation_of_primes(2), 0);
        assert_eq!(Solution::summation_of_primes(3), 2);
        assert_eq!(Solution::summation_of_primes(4), 5);
        assert_eq!(Solution::summation_of_primes(5), 5);
        assert_eq!(Solution::summation_of_primes(10), 17);
        assert_eq!(Solution::summation_of_primes(2000000), 142913828922);
    }
}
