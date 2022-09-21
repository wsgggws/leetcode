// Amicable numbers
// Problem 21
// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

// Evaluate the sum of all the amicable numbers under 10000.

pub struct Solution {}

impl Solution {
    pub fn amicable_numbers(num: u64) -> u64 {
        if num > 10000 {
            panic!("Plz set n <= 10000");
        }

        (10..num)
            .filter(|&number| Solution::is_amicable_number(number))
            .sum::<u64>()
    }

    fn is_amicable_number(number: u64) -> bool {
        let divisors_sum = Solution::get_divisors_sum(number);
        if divisors_sum != number && Solution::get_divisors_sum(divisors_sum) == number {
            true
        } else {
            false
        }
    }

    fn get_divisors_sum(number: u64) -> u64 {
        (1..=number/2)
            .filter(|&divisor| number % divisor == 0)
            .sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amicable_numbers_test() {
        assert_eq!(Solution::amicable_numbers(20), 0);
        assert_eq!(Solution::amicable_numbers(300), 504);
        assert_eq!(Solution::amicable_numbers(10000), 31626);
    }
}
