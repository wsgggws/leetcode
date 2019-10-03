// 50. Pow(x, n)
// Medium

// Implement pow(x, n), which calculates x raised to the power n (xn).

// Example 1:

// Input: 2.00000, 10
// Output: 1024.00000
// Example 2:

// Input: 2.10000, 3
// Output: 9.26100
// Example 3:

// Input: 2.00000, -2
// Output: 0.25000
// Explanation: 2-2 = 1/22 = 1/4 = 0.25
// Note:

// -100.0 < x < 100.0
// n is a 32-bit signed integer, within the range [−231, 231 − 1]

pub struct Solution {}


impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // 直接调用内置的库, 能够快速通过.
        x.powi(n)

        // Time Limit Exceeded Error: Last executed input:
        // 0.00001
        // 2147483647
        // let mut result = 1.0;
        // let mut x = x;
        // let mut counts: i64 = n as i64;
        // if counts < 0 {
        //     x = 1.0/x;
        //     counts = -counts;
        // }
        // for _ in 0..counts{
        //     result *= x;
        // }
        // result


        // // 手动实现二分法快速幕, 能通过
        // if n == 0 { return 1.0; }
        // let mut result = 1.0;
        // // 最小最大的整数需要考虑, 所以转换成i64
        // let mut n: i64 = n as i64;
        // let mut x = x;
        // if n < 0 {
        //     n = -n;
        //     x = 1.0/x;
        // }
        // // x**n = x**2**(a-1) ... * ... *  x**2**0  其中a为n二进制里为1的从右到左的index
        // while n > 0 {
        //     if n & 1 == 1 { result *= x; }
        //     x *= x;
        //     n >>= 1;
        // }
        // result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_pow_test() {
        assert!((Solution::my_pow(2.00000, 0) - 1.0).abs() < 1e-5);
        assert!((Solution::my_pow(2.00000, 10) - 1024.00000).abs() < 1e-5);
        assert!((Solution::my_pow(2.10000, 3) - 9.26100).abs() < 1e-5);
        assert!((Solution::my_pow(2.00000, -2) - 0.25000).abs() < 1e-5);
    }
}
