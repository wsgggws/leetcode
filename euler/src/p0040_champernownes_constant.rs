// Champernowne's constant
// Problem 40
// An irrational decimal fraction is created by concatenating the positive integers:

// 0.123456789101112131415161718192021...

// It can be seen that the 12th digit of the fractional part is 1.

// If dn represents the nth digit of the fractional part, find the value of the following expression.

// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

pub struct Solution {}

impl Solution {
    pub fn champernowners_constant() -> i32 {
        (0..7)
            .map(|x| 10i32.pow(x))
            .map(|x| Solution::get_dn(x))
            .fold(1, |result, x| result * x)
    }

    fn get_dn(number: i32) -> i32 {
        // remainder_number 用于表示过了第n位（1位只有1~9, 2位只有10~99）
        // 第index位所表示的数字
        let mut index: i32 = 0;
        let mut remainder_number = number;
        for i in 0i32..7 {
            remainder_number -= 9 * 10i32.pow(i as u32) * (i+1);
            if remainder_number <= 0 {
                index = i;
                break;
            }
        }
        remainder_number += 9 * 10i32.pow(index as u32) * (index + 1);

        // nth 表示过第index位的第nth个数
        // remainder表示第index+1位的第remainder+1个数字,
        // 当它为0时，恰好在index位第nth个数的最后一位
        // 否则，在index+1位的第remainder位
        let nth = remainder_number / (index + 1);
        let remainder = remainder_number % (index + 1);
        if remainder == 0 {
            let num = 10i32.pow(index as u32) - 1 + nth;
            num.to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as i32
        } else {
            let num = 10i32.pow(index as u32) - 1 + nth + 1;
            num.to_string()
                .chars()
                .nth((remainder-1) as usize)
                .unwrap()
                .to_digit(10)
                .unwrap() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn champernowners_constant_test() {
        // [1, 1, 5, 3, 7, 2, 1]
        assert_eq!(Solution::champernowners_constant(), 210);
    }

    #[test]
    fn get_dn_test() {
        assert_eq!(Solution::get_dn(1), 1);
        assert_eq!(Solution::get_dn(9), 9);
        assert_eq!(Solution::get_dn(10), 1);
        assert_eq!(Solution::get_dn(11), 0);
        assert_eq!(Solution::get_dn(12), 1);
        assert_eq!(Solution::get_dn(13), 1);
    }
}
