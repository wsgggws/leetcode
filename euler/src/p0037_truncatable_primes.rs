// Truncatable primes
// Problem 37
// The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.

// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

pub struct Solution {}

use std::collections::HashSet;
impl Solution {

    pub fn truncatable_primes(n: usize) -> u64 {
        // 题目要求是前11位的和，要注意申题，不是第11位!!!
        (11..)
            .filter(|&x| Solution::is_truncatable_prime(x))
            .take(n)
            .sum()
    }

    pub fn get_nth_truncatable_prime(n: usize) -> u64 {
        (11..)
            .filter(|&x| Solution::is_truncatable_prime(x))
            .nth(n-1)
            .unwrap()
    }

    fn is_truncatable_prime(number: u64) -> bool {
        Solution::get_truncatable_numbers(number)
            .iter()
            .all(|&x| Solution::is_prime(x))
    }

    fn is_prime(num: u64) -> bool {
        // NOTE 注意对1做处理
        if num == 1 {
            return false;
        }
        if num == 2 || num == 3 {
            return true;
        }
        !(2..=(num as f64).sqrt().ceil() as u64).any(|value| num % value == 0)
    }

    fn get_truncatable_numbers(number: u64) -> HashSet<u64> {
        let mut truncatable_numbers: HashSet<u64> = HashSet::new();
        truncatable_numbers.insert(number);

        for i in 1..number.to_string().len() {
            truncatable_numbers.insert(number / 10u64.pow(i as u32));
            truncatable_numbers.insert(number % 10u64.pow(i as u32));
        }
        truncatable_numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncatable_primes_test() {
        assert_eq!(Solution::truncatable_primes(1), 23);
        assert_eq!(Solution::truncatable_primes(2), 60);
        assert_eq!(Solution::truncatable_primes(11), 748317);
    }

    #[test]
    fn get_nth_truncatable_prime_test() {
        assert_eq!(Solution::get_nth_truncatable_prime(1), 23);
        assert_eq!(Solution::get_nth_truncatable_prime(2), 37);
        assert_eq!(Solution::get_nth_truncatable_prime(3), 53);
        assert_eq!(Solution::get_nth_truncatable_prime(4), 73);
        assert_eq!(Solution::get_nth_truncatable_prime(5), 313);
        assert_eq!(Solution::get_nth_truncatable_prime(6), 317);
        assert_eq!(Solution::get_nth_truncatable_prime(7), 373);
        assert_eq!(Solution::get_nth_truncatable_prime(8), 797);
        assert_eq!(Solution::get_nth_truncatable_prime(9), 3137);
        assert_eq!(Solution::get_nth_truncatable_prime(10), 3797);
        assert_eq!(Solution::get_nth_truncatable_prime(11), 739397);
    }

    #[test]
    fn is_truncatable_prime_test() {
        assert_eq!(Solution::is_truncatable_prime(3796), false);
        assert_eq!(Solution::is_truncatable_prime(3797), true);
    }

    #[test]
    fn get_truncatable_numbers_test() {
        assert_eq!(Solution::get_truncatable_numbers(11), [11, 1].iter().cloned().collect::<HashSet<u64>>());
        assert_eq!(Solution::get_truncatable_numbers(3797), [3797, 379, 7, 37, 97, 3, 797].iter().cloned().collect::<HashSet<u64>>());
    }
}
