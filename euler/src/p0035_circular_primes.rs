// Circular primes
//   Show HTML problem content
// Problem 35
// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

// How many circular primes are there below one million?

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn circular_primes(n: u64) -> usize {
        let primes = (2..n).filter(|&x| Solution::is_prime(x)).collect::<HashSet<u64>>();
        (2..n)
            .filter(|&x| Solution::is_circular_prime(x, &primes))
            .count()
    }

    fn is_prime(num: u64) -> bool {
        if num == 2 || num == 3 {
            return true;
        }
        !(2..=(num as f64).sqrt().ceil() as u64).any(|value| num % value == 0)
    }

    fn is_circular_prime(x: u64, primes: &HashSet<u64>) -> bool {
        let circular_numbers = Solution::get_circular_numbers(x);
        for &circular_number in circular_numbers.iter() {
            if !primes.contains(&circular_number) {
                return false
            }
        }
        true
    }

    fn get_circular_numbers(x: u64) -> Vec<u64> {
        let strs = x.to_string();
        let mut circular_numbers: Vec<u64> = vec![];
        for i in 0..strs.len() {
            let num = format!("{}{}", &strs[i..], &strs[..i]).parse().unwrap();
            circular_numbers.push(num);
        }
        circular_numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circular_primes_test() {
        assert_eq!(Solution::circular_primes(100), 13);
        assert_eq!(Solution::circular_primes(1000000), 55);
    }

    #[test]
    fn is_prime_test() {
        assert_eq!(Solution::is_prime(2), true);
        assert_eq!(Solution::is_prime(3), true);
        assert_eq!(Solution::is_prime(4), false);
        assert_eq!(Solution::is_prime(79), true);
        assert_eq!(Solution::is_prime(91), false);
    }

    #[test]
    fn get_circular_numbers_test() {
        assert_eq!(Solution::get_circular_numbers(79), vec![79, 97]);
        assert_eq!(Solution::get_circular_numbers(789), vec![789, 897, 978]);
    }

    #[test]
    fn is_circular_prime_test() {
        let primes = (2..100).filter(|&x| Solution::is_prime(x)).collect::<HashSet<u64>>();
        assert_eq!(Solution::is_circular_prime(79, &primes), true);
        assert_eq!(Solution::is_circular_prime(19, &primes), false);
    }
}
