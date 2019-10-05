// 69. Sqrt(x)
// Easy

// Implement int sqrt(int x).

// Compute and return the square root of x, where x is guaranteed to be a non-negative integer.

// Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.

// Example 1:

// Input: 4
// Output: 2
// Example 2:

// Input: 8
// Output: 2
// Explanation: The square root of 8 is 2.82842..., and since 
//              the decimal part is truncated, 2 is returned.

pub struct Solution {}


impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (x as f64).sqrt() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_sqrt_test() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
        assert_eq!(Solution::my_sqrt(10), 3);
    }
}
