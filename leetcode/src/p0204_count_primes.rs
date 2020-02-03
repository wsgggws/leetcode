// 204. Count Primes
// Easy

// Count the number of prime numbers less than a non-negative number, n.

// Example:

// Input: 10
// Output: 4
// Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut primes: HashSet<i64> = HashSet::new();
        let mut count = 0;
        for index in 2_i64..n as i64 {
            if primes.contains(&index) {
                continue;
            }
            count += 1;
            let mut j = index * index;
            while j < n as i64 {
                primes.insert(j);
                j += index;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn count_primes_test() {
        assert_eq!(Solution::count_primes(2), 0);
        assert_eq!(Solution::count_primes(3), 1);
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(499979), 41537);
    }
}
