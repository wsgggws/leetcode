// Smallest multiple
// Problem 5
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn smallest_multiple(n: u64) -> u64 {
        let mut prime_divicotors: HashMap<u64, u32> = HashMap::new();
        // 任何合数都可分解成质数的乘积，故计算质数之积即可
        for number in 2..=n {
            Solution::get_prime_divicotors(number, &mut prime_divicotors);
        }
        let mut result = 1u64;
        for (key, value) in prime_divicotors.iter() {
            for _ in 0..*value {
                result *= key;
            }
        }
        result
    }

    fn get_prime_divicotors(number: u64, prime_divicotors: &mut HashMap<u64, u32>) {
        let primes: [u64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
        let mut number = number;
        for &prime in primes.iter() {
            if number % prime == 0 {
                let mut count_prime = 0u32;
                while number % prime == 0 {
                    number = number / prime;
                    count_prime += 1u32;
                }
                let value = prime_divicotors.get(&prime).unwrap_or(&0);
                if count_prime > *value {
                    prime_divicotors.insert(prime, count_prime);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_multiple_test() {
        assert_eq!(Solution::smallest_multiple(10), 2520);
        assert_eq!(Solution::smallest_multiple(20), 232792560);
    }
}
