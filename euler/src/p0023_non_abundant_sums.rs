// Non-abundant sums
// Problem 23
// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

// A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.

// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.

// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn non_abundant_sums() -> u32 {
        let abundants: Vec<u32> = Solution::get_abundants();
        let mut two_abundants_sums: HashSet<u32> = HashSet::new();
        let lens = abundants.len();
        for i in 0..lens {
            for j in 0..lens {
                if abundants[i] + abundants[j] < 28123 {
                    two_abundants_sums.insert(abundants[i] + abundants[j]);
                }
            }
        }
        (1..28123)
            .filter(|num| two_abundants_sums.get(num).is_none())
            .sum::<u32>()
    }

    fn get_abundants() -> Vec<u32> {
        (12..28123)
            .filter(|&num| Solution::is_abundant(num))
            .collect::<Vec<u32>>()
    }

    fn is_abundant(number: u32) -> bool {
        (1..=number/2)
            .filter(|&num| number % num == 0)
            .sum::<u32>()
            .gt(&number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_abundant_sums_test() {
        assert_eq!(Solution::non_abundant_sums(), 4179871);
    }
}
