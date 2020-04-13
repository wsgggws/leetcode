// 371. Sum of Two Integers
// Easy

// Calculate the sum of two integers a and b, but you are not allowed to use the operator + and -.

// Example 1:

// Input: a = 1, b = 2
// Output: 3
// Example 2:

// Input: a = -2, b = 3
// Output: 1

pub struct Solution {}

// 正数的补码和原码相同，负数的补码为除了第一位符号位之外，其他位0变1,1变0，并且最后再加1
// 以 -1 + 2 = 1 为例
// 11111111 ^ 00000010 = 11111101, (11111111 & 00000010) << 1 = 00000100
// 11111101 ^ 00000100 = 11111001, (11111101 & 00000100) << 1 = 00001000
// 11111001 ^ 00001000 = 11110001, (11111001 & 00001000) << 1 = 00010000
// ...
// 10000001 ^ 10000000 = 00000001, (10000001 & 10000000) << 1 = 00000000
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let temp = a ^ b;
            b = (a & b) << 1;
            a = temp;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sum_test() {
        assert_eq!(Solution::get_sum(1, 2), 3);
        assert_eq!(Solution::get_sum(-2, 3), 1);
    }
}
