// Pandigital multiples
// Problem 38
// Take the number 192 and multiply it by each of 1, 2, and 3:

// 192 × 1 = 192
// 192 × 2 = 384
// 192 × 3 = 576
// By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)

// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).

// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn get_max_pandigital_multiple(n: u64) -> u64 {
        let mut maxs_pandigital = u64::MIN;
        for num in 3..n {
            for i in 2..9 {
                let number = Solution::get_product_result(num, i);
                if Solution::is_pandigitial_multiple(number) && number > maxs_pandigital {
                    // println!("num: {}, i: {}, number: {}", num, i, number);
                    maxs_pandigital = number;
                }
            }
        }
        maxs_pandigital
    }

    fn get_product_result(number: u64, i: u64) -> u64 {
        let mut strs: String = String::new();
        for multiplier in 1..=i {
            strs = strs + &format!("{}", number*multiplier);
        }
        if strs.len() > 9 {
            return 0;
        }
        strs.parse::<u64>().unwrap()
    }

    fn is_pandigitial_multiple(number: u64) -> bool {
        let hash_set = number.to_string().chars().collect::<HashSet<char>>();
        if number.to_string().len() != 9 || hash_set.contains(&'0') || hash_set.len() != 9 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pandigital_multiples_test() {
        // 200以内为 9, 1..=5, 918273645
        assert_eq!(Solution::get_max_pandigital_multiple(200), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(1000), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(2000), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(3000), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(4000), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(5000), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(6000), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(7000), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(8000), 918273645);
        assert_eq!(Solution::get_max_pandigital_multiple(9000), 918273645);
        // num: 9267, i: 2, number: 926718534
        // num: 9273, i: 2, number: 927318546
        // num: 9327, i: 2, number: 932718654
        assert_eq!(Solution::get_max_pandigital_multiple(10000), 932718654);
    }
}
