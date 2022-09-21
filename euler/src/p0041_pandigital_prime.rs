// Pandigital prime
// Problem 41
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

// What is the largest n-digit pandigital prime that exists?

pub struct Solution {}

use std::char;
impl Solution {
    pub fn pandigital_prime() -> u64 {
        // TODO 测试出来的，2.。10000000 超时
        (7000000..8000000)
            .filter(|&x| Solution::is_prime(x))
            .filter(|&x| Solution::is_pandigital_prime(x))
            .max_by_key(|x| x.to_string().len())
            .unwrap()
    }

    fn is_pandigital_prime(prime: u64) -> bool {
        let mut chars1 = prime.to_string().chars().collect::<Vec<char>>();
        chars1.sort();
        let chars2 = (1u32..=prime.to_string().len() as u32)
            .map(|x| char::from_digit(x, 10).unwrap())
            .collect::<Vec<char>>();
        chars1 == chars2
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
    fn pandigital_prime_test() {
        assert_eq!(Solution::pandigital_prime(), 7652413);
    }
}
