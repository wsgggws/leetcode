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
        // Error: Last executed input:
        // 0.00001
        // 2147483647

        // let mut flag = true;
        // if n < 0 { flag = false; }
        // let mut result = 1.0;
        // let mut x = x;
        // let mut counts = n;
        // if !flag {
        //     x = 1.0/x;
        //     counts += 1;
        // }else{
        //     x = 1.0*x;
        //     counts -= 1;
        // }
        // result *= x;
        // if counts < 0 { counts = -counts; }
        // for _ in 0..counts{
        //     result *= x;
        // }
        // result

        // 直接调用内置的库, 能够快速通过.
        x.powi(n)
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
