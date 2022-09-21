// Power digit sum
// Problem 16
// 215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 21000?
// extern crate bigint;
// use bigint::U256;

// pub struct Solution {}

// impl Solution {
//     pub fn power_digit_sum(n: usize) -> u64 {
//         let mut val: U256 = 1.into();
//         for _ in 0..n { val = val * 2.into() }
//         let var = format!("{}", val);
//         println!("var: {}", var);
//         let result = var.chars().map(|num| num.to_digit(10).unwrap() as u64).sum::<u64>();
//         result
//     }
// }

pub struct Solution {}

impl Solution {
    pub fn power_digit_sum(n: usize) -> u32 {
        let mut result = String::from("1");
        for _ in 0..n {
            result = Solution::mul_2_for_string(&result);
        }
        result.chars()
            .map(|num| num.to_digit(10).unwrap())
            .sum::<u32>()
    }

    fn mul_2_for_string(number: &String) -> String {
        let mut carry = 0u32;
        let mut result = String::new();
        for num in number.chars().rev().map(|num| num.to_digit(10).unwrap()) {
            result.push_str(&((num * 2 + carry) % 10).to_string());
            carry = if (num * 2 + carry) >= 10 { 1u32 } else { 0u32 };
        }
        if carry > 0 {
            result.push_str(&carry.to_string());
        }
        result.chars()
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_digit_sum_test() {
        // 曾经使用crate bigint = "4" 去计算，bigint::U256 最多只能计算到2**255,
        // 故只能放弃自己实现‘字符串’ * 2, 算1000次
        // var: 1
        // var: 2
        // var: 4
        // var: 32768
        // var: 1606938044258990275541962092341162602522202993782792835301376
        // var: 1809251394333065553493296640760748560207343510400633813116524750123642650624
        // var: 57896044618658097711785492504343953926634992332820282019728792003956564819968
        assert_eq!(Solution::power_digit_sum(0), 1);
        assert_eq!(Solution::power_digit_sum(1), 2);
        assert_eq!(Solution::power_digit_sum(2), 4);
        assert_eq!(Solution::power_digit_sum(15), 26);
        assert_eq!(Solution::power_digit_sum(200), 256);
        assert_eq!(Solution::power_digit_sum(250), 286);
        assert_eq!(Solution::power_digit_sum(255), 377);
        assert_eq!(Solution::power_digit_sum(1000), 1366);
    }
}
