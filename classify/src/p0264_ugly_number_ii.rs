// 264. Ugly Number II
// Medium

// Write a program to find the n-th ugly number.

// Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.

// Example:

// Input: n = 10
// Output: 12
// Explanation: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10 ugly numbers.
// Note:

// 1 is typically treated as an ugly number.
// n does not exceed 1690.

pub struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_numbers: Vec<i32> = vec![1];
        let (mut index_2, mut index_3, mut index_5) = (0, 0, 0);
        for _ in 1..n {
            let ugly_2 = ugly_numbers[index_2] * 2;
            let ugly_3 = ugly_numbers[index_3] * 3;
            let ugly_5 = ugly_numbers[index_5] * 5;
            let cur_mins = i32::min(ugly_2, i32::min(ugly_3, ugly_5));
            if cur_mins == ugly_2 { index_2 += 1 }
            if cur_mins == ugly_3 { index_3 += 1 }
            if cur_mins == ugly_5 { index_5 += 1 }
            ugly_numbers.push(cur_mins);
        }
        ugly_numbers[ugly_numbers.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_ugly_number_test() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }
}
