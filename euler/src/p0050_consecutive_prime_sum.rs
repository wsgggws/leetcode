// Consecutive prime sum
// Problem 50
// The prime 41, can be written as the sum of six consecutive primes:

// 41 = 2 + 3 + 5 + 7 + 11 + 13
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.

// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.

// Which prime, below one-million, can be written as the sum of the most consecutive primes?

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn consecutive_prime_sum(below: u64) -> u64 {
        let primes_set = (2..below).filter(|&x| Solution::is_prime(x)).collect::<HashSet<u64>>();
        let primes = (2..below).filter(|&x| Solution::is_prime(x)).collect::<Vec<u64>>();
        let mut primes_sum: Vec<(usize, usize, u64)> = vec![];
        for i in 0..primes.len() {
            for j in i..primes.len() {
                let sum_i_j = primes[i..=j].iter().sum::<u64>();
                if sum_i_j < below {
                    primes_sum.push((i, j, sum_i_j));
                } else {
                    break;
                }
            }
        }
        let result = primes_sum.iter()
            .filter(|&(_, _, sum_i_j)| primes_set.contains(&sum_i_j))
            .max_by_key(|x| x.1 - x.0)
            .unwrap();
        // println!("result: {:?}", result);
        result.2
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
    fn consecutive_prime_sum_test() {
        // result: (0, 5, 41), 第0位素数到第5位素数，共6位，和为41
        assert_eq!(Solution::consecutive_prime_sum(100), 41);
        // result: (3, 23, 953)
        assert_eq!(Solution::consecutive_prime_sum(1000), 953);
        // result: (3, 545, 997651)
        assert_eq!(Solution::consecutive_prime_sum(1000000), 997651);
    }
}
