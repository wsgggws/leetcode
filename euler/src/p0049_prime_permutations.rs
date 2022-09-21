// Prime permutations
// Problem 49
// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.

// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.

// What 12-digit number do you form by concatenating the three terms in this sequence?

pub struct Solution {}

use std::collections::HashSet;
use permutohedron::heap_recursive;
impl Solution {
    pub fn prime_permutations() -> u64 {
        let primes_set = (1001..9997).filter(|&x| Solution::is_prime(x)).collect::<HashSet<u64>>();
        let primes = (1001..9997).filter(|&x| Solution::is_prime(x)).collect::<Vec<u64>>();
        for &prime in primes.iter() {
            let prime_numbers = Solution::get_sorted_prime_numbers(prime, &primes_set);
            if let Some((num1, num2, num3)) = Solution::get_three_number(&prime_numbers) {
                if num1 != 1487 {
                    return num1 * 100000000 + num2 * 10000 + num3
                }
            }
        }
        1
    }

    fn get_three_number(prime_numbers: &Vec<u64>) -> Option<(u64, u64, u64)> {
        if prime_numbers.len() == 0 {
            return None
        }
        // TODO 这里对于 1487, 4817, 8147 没有处理好
        for gap in 1..=prime_numbers.len()/3 {
            for i in 0..prime_numbers.len() {
                if i + 2*gap < prime_numbers.len() && prime_numbers[i+gap] - prime_numbers[i] == prime_numbers[i+2*gap] - prime_numbers[i+gap] {
                    return Some((prime_numbers[i], prime_numbers[i+gap], prime_numbers[i+2*gap]));
                }
            }
        }
        None
    }

    fn is_prime(num: u64) -> bool {
        if num == 2 || num == 3 {
            return true;
        }
        !(2..=(num as f64).sqrt().ceil() as u64)
            .any(|value| num % value == 0)
    }

    fn get_sorted_prime_numbers(prime: u64, primes_set: &HashSet<u64>) -> Vec<u64> {
        let mut data = prime.to_string().chars().collect::<Vec<char>>();
        let mut permutations = Vec::new();
        heap_recursive(&mut data, |permutation| {
            permutations.push(permutation.to_vec())
        });
        let mut tmp: HashSet<u64> = HashSet::new();
        for permutation in permutations.iter() {
            let number = permutation.into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .fold(0u64, |p, a| p * 10 + a as u64);
            if primes_set.contains(&number) {
                tmp.insert(number);
            }
        }
        if tmp.len() >= 3{
            let mut prime_numbers: Vec<u64> = vec![];
            for &x in tmp.iter() {
                prime_numbers.push(x);
            }
            prime_numbers.sort();
            prime_numbers
        } else {
            vec![]
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_permutations_test() {
        assert_eq!(Solution::prime_permutations(), 296962999629);
    }
}
