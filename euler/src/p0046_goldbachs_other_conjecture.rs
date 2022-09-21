// Goldbach's other conjecture
// Problem 46
// It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.

// 9 = 7 + 2×12
// 15 = 7 + 2×22
// 21 = 3 + 2×32
// 25 = 7 + 2×32
// 27 = 19 + 2×22
// 33 = 31 + 2×12

// It turns out that the conjecture was false.

// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn goldbachs_other_conjecture() -> i128 {
        let primes = (2..10000).filter(|&x| Solution::is_prime(x)).collect::<HashSet<i128>>();
        (35..)
            .step_by(2)
            .filter(|&x| !Solution::can_be_written_as(x, &primes))
            .take(1)
            .next()
            .unwrap()
    }

    fn is_prime(num: i128) -> bool {
        if num == 2 || num == 3 {
            return true;
        }
        !(2..=(num as f64).sqrt().ceil() as i128)
            .any(|value| num % value == 0)
    }

    fn can_be_written_as(number: i128, primes: &HashSet<i128>) -> bool {
        // NOTE 137 = 137 + 2 * 0 * 0 也可以，所以得从0开始, 不是从1开始
        (0..1000)
            .filter(|&x| number - 2*(x*x) >= 2)
            .map(|x| number - 2 * x * x)
            .rev()
            .any(|prime| primes.contains(&prime))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goldbachs_other_conjecture_test() {
        assert_eq!(Solution::goldbachs_other_conjecture(), 5777);
    }

    #[test]
    fn can_be_written_as_test() {
        let primes = (2..10000).filter(|&x| Solution::is_prime(x)).collect::<HashSet<i128>>();
        assert_eq!(Solution::can_be_written_as(33, &primes), true);
        assert_eq!(Solution::can_be_written_as(35, &primes), true);
        assert_eq!(Solution::can_be_written_as(137, &primes), true);
    }
}
