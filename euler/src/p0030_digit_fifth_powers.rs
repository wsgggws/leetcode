// Digit fifth powers
// Problem 30
// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:

// 1634 = 14 + 64 + 34 + 44
// 8208 = 84 + 24 + 04 + 84
// 9474 = 94 + 44 + 74 + 44
// As 1 = 14 is not a sum it is not included.

// The sum of these numbers is 1634 + 8208 + 9474 = 19316.

// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

pub struct Solution {}

impl Solution {
    pub fn digits_fifth_powers() -> u32 {

        // TODO 如何确定这个上限为200000?
        // (10..=200000).filter(|&num| num == Solution::get_sum_of_digits_fifth_powers(num)).count() as u32

        (10..=200000).
            filter(|&num| num == Solution::get_sum_of_digits_fifth_powers(num))
            .sum::<u32>()
    }

    fn get_sum_of_digits_fifth_powers(num: u32) -> u32 {
        let mut number = num;
        let mut sums = 0;
        while number > 0 {
            sums += (number % 10).pow(5);
            number /= 10;
        }
        sums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_fifth_powers_test() {
        assert_eq!(Solution::digits_fifth_powers(), 443839);
    }
}
