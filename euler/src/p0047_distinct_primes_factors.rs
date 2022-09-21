// Distinct primes factors
// Problem 47
// The first two consecutive numbers to have two distinct prime factors are:

// 14 = 2 × 7
// 15 = 3 × 5

// The first three consecutive numbers to have three distinct prime factors are:

// 644 = 2² × 7 × 23
// 645 = 3 × 5 × 43
// 646 = 2 × 17 × 19.

// Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn distinct_primes_factors() -> u64 {
        let primes = (2..10000)
            .filter(|&x| Solution::is_prime(x))
            .collect::<Vec<u64>>();
        let prime_factors: Vec<(u64, usize)> = (1000..1000000)
            .map(|x| Solution::get_distinct_primes_number(x, &primes))
            .collect::<Vec<(u64, usize)>>();

        for i in 0..prime_factors.len() {
            if i + 3 < prime_factors.len()
                && prime_factors[i].1 == 4
                && prime_factors[i + 1].1 == 4
                && prime_factors[i + 2].1 == 4
                && prime_factors[i + 3].1 == 4
            {
                return prime_factors[i].0;
            }
        }
        0
    }

    fn is_prime(num: u64) -> bool {
        if num == 2 || num == 3 {
            return true;
        }
        !(2..=(num as f64).sqrt().ceil() as u64).any(|value| num % value == 0)
    }

    fn get_distinct_primes_number(num: u64, primes: &Vec<u64>) -> (u64, usize) {
        let mut hash_set = HashSet::new();
        let mut number = num;
        for &prime in primes.iter() {
            if number > 1 && number % prime == 0 {
                while number % prime == 0 {
                    number /= prime;
                }
                hash_set.insert(prime);
            }
        }
        (num, hash_set.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_primes_factors_test() {
        assert_eq!(Solution::distinct_primes_factors(), 134043);
    }
}
