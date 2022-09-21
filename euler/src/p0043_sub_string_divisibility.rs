// Sub-string divisibility
// Problem 43
// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.

// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:

// d2d3d4=406 is divisible by 2
// d3d4d5=063 is divisible by 3
// d4d5d6=635 is divisible by 5
// d5d6d7=357 is divisible by 7
// d6d7d8=572 is divisible by 11
// d7d8d9=728 is divisible by 13
// d8d9d10=289 is divisible by 17
// Find the sum of all 0 to 9 pandigital numbers with this property.

pub struct Solution {}

use permutohedron::heap_recursive;
impl Solution {
    // 利用10位数字的全排列，时间复杂度为10! == 3628800
    pub fn sub_string_divisibility() -> u64 {
        let mut data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut permutations = Vec::new();
        heap_recursive(&mut data, |permutation| {
            permutations.push(permutation.to_vec())
        });
        let mut sum = 0;
        for permutation in permutations.iter() {
            let number = permutation.iter().fold(0, |result, x| result * 10 + x);
            if Solution::is_sub_string_divisibility(number) {
                sum += number;
            }
        }
        sum
    }

    fn is_sub_string_divisibility(number: u64) -> bool {
        // 过滤掉最左边数字为0的number
        if number < 1_000_000_000 {
            return false;
        }
        let mut n = number;
        let primes = [17, 13, 11, 7, 5, 3, 2];
        for i in 0..=6 {
            let sub_3_number = n % 1000;
            if sub_3_number % primes[i] != 0 {
                return false;
            }
            n /= 10;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_string_divisibility_test() {
        // number: 1406357289
        // number: 4106357289
        // number: 4160357289
        // number: 1460357289
        // number: 1430952867
        // number: 4130952867
        assert_eq!(Solution::sub_string_divisibility(), 16695334890);
    }

    #[test]
    fn is_sub_string_divisibility_test() {
        assert_eq!(Solution::is_sub_string_divisibility(1406357289), true);
        assert_eq!(Solution::is_sub_string_divisibility(1406359287), false);
    }
}

// 复杂度太高
// use std::collections::HashSet;
// impl Solution {
//     pub fn sub_string_divisibility() -> u64 {
//         let mut result: u64 = 0;
//         for &div_2 in Solution::get_product_by_div(2).iter() {
//             for &div_3 in Solution::get_product_by_div(3).iter() {
//                 for &div_5 in Solution::get_product_by_div(5).iter() {
//                     for &div_7 in Solution::get_product_by_div(7).iter() {
//                         for &div_11 in Solution::get_product_by_div(11).iter() {
//                             for &div_13 in Solution::get_product_by_div(13).iter() {
//                                 for &div_17 in Solution::get_product_by_div(17).iter() {
//                                     let numbers = Solution::get_numbers_from_parts(div_2, div_3, div_5, div_7, div_11, div_13, div_17);
//                                     for &number in numbers.iter() {
//                                         if Solution::is_prime(number) {
//                                             result += number;
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         result
//     }

//     fn is_prime(num: u64) -> bool {
//         if num == 2 || num == 3 {
//             return true;
//         }
//         !(2..=(num as f64).sqrt().ceil() as u64)
//             .any(|value| num % value == 0)
//     }

//     fn get_numbers_from_parts(div_2: u64, div_3: u64, div_5: u64, div_7: u64, div_11: u64, div_13: u64, div_17: u64) -> Vec<u64> {
//         let a_str = div_2.to_string() 
//             + &div_3.to_string() 
//             + &div_5.to_string() 
//             + &div_7.to_string() 
//             + &div_11.to_string()
//             + &div_13.to_string()
//             + &div_17.to_string();
//         (1..=9)
//             .map(|x| x.to_string() + &a_str)
//             .map(|strs| strs.parse::<u64>().unwrap())
//             .filter(|&num| Solution::is_different_digits(num, 10))
//             .collect::<Vec<u64>>()
//     }

//     fn get_product_by_div(div: u64) -> Vec<u64> {
//         (100/div..=1000/div)
//             .map(|x| x * div)
//             .filter(|&x| Solution::is_different_digits(x, 3))
//             .collect()
//     }

//     fn is_different_digits(number: u64, max_length: usize) -> bool {
//         number.to_string().len() == max_length 
//             && number.to_string().chars().collect::<HashSet<char>>().len() == max_length
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn sub_string_divisibility_test() {
//         assert_eq!(Solution::sub_string_divisibility(), 1);
//     }

//     #[test]
//     fn get_product_by_div_test() {
//         // 在有限的时间内根本计算不出来 328 * 228 * 136 * 93
//         assert_eq!(Solution::get_product_by_div(2).len(), 328);
//         assert_eq!(Solution::get_product_by_div(3).len(), 228);
//         assert_eq!(Solution::get_product_by_div(5).len(), 136);
//         assert_eq!(Solution::get_product_by_div(7).len(), 93);
//         assert_eq!(Solution::get_product_by_div(11).len(), 64);
//         assert_eq!(Solution::get_product_by_div(13).len(), 50);
//         assert_eq!(Solution::get_product_by_div(17).len(), 39);
//     }

//     #[test]
//     fn is_different_digits_test() {
//         assert_eq!(Solution::is_different_digits(1000, 3), false);
//         assert_eq!(Solution::is_different_digits(101, 3), false);
//         assert_eq!(Solution::is_different_digits(102, 3), true);
//     }
// }
