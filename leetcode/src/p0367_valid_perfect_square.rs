// 367. Valid Perfect Square
// Easy

// Given a positive integer num, write a function which returns True if num is a perfect square else False.

// Note: Do not use any built-in library function such as sqrt.

// Example 1:

// Input: 16
// Output: true
// Example 2:

// Input: 14
// Output: false

pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        // 使用暴力法
        // if num == 1 || num == 0 { return true; }
        // let mut i = 1_i64;
        // while i <= (num as i64)/2 {
        //     if i * i == num as i64 { return true; }
        //     if i * i > num as i64 { return false; }
        //     i += 1;
        // }
        // false

        // 使用二分法
        if num == 1 || num == 0 {
            return true;
        }
        let mut start: i64 = 0;
        let mut end: i64 = (num as i64) / 2;
        while start <= end {
            let mid = (start + end) / 2;
            if mid * mid == num as i64 {
                return true;
            } else if mid * mid < num as i64 {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_perfect_square_test() {
        assert_eq!(Solution::is_perfect_square(-1), false);
        assert_eq!(Solution::is_perfect_square(0), true);
        assert_eq!(Solution::is_perfect_square(1), true);
        assert_eq!(Solution::is_perfect_square(2), false);
        assert_eq!(Solution::is_perfect_square(4), true);
        assert_eq!(Solution::is_perfect_square(14), false);
        assert_eq!(Solution::is_perfect_square(16), true);
        assert_eq!(Solution::is_perfect_square(808201), true);
    }
}
