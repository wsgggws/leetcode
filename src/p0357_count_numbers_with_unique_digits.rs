// 357. Count Numbers with Unique Digits
// Medium

// Given a non-negative integer n, count all numbers with unique digits, x, where 0 ≤ x < 10n.

// Example:

// Input: 2
// Output: 91 
// Explanation: The answer should be the total numbers in the range of 0 ≤ x < 100, 
//              excluding 11,22,33,44,55,66,77,88,99

pub struct Solution {}

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let array = [9, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut counts = 1;
        let mut tmp = 1;
        for i in 0..i32::min(n, 10) {
            tmp *= array[i as usize];
            counts += tmp;
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_numbers_with_unique_digits_test() {
        assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
        assert_eq!(Solution::count_numbers_with_unique_digits(1), 10);
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
        assert_eq!(Solution::count_numbers_with_unique_digits(3), 739);
        assert_eq!(Solution::count_numbers_with_unique_digits(10), 8877691);
        assert_eq!(Solution::count_numbers_with_unique_digits(13), 8877691);
    }
}
