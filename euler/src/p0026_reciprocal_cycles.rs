// Reciprocal cycles
// Problem 26
// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:

// 1/2	= 	0.5
// 1/3	= 	0.(3)
// 1/4	= 	0.25
// 1/5	= 	0.2
// 1/6	= 	0.1(6)
// 1/7	= 	0.(142857)
// 1/8	= 	0.125
// 1/9	= 	0.(1)
// 1/10	= 	0.1
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.

// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

pub struct Solution {}

impl Solution {
    pub fn reciprocal_cycles(n: u32) -> usize {
        // 题目要求是计算d, 而不是 longest recurring cycle, 所以加了层enumerate来跟踪d, 结果也需要+1
        (1..=n)
            .map(|x| Solution::get_reciprocal_cycles(x))
            .enumerate()
            .max_by_key(|(_, strs)| strs.len())
            .unwrap().0 + 1
    }

    fn get_reciprocal_cycles(number: u32) -> String {
        let mut remainderes: Vec<u32> = vec![1];
        let mut quotientes: Vec<u32> = vec![];
        let mut remainder = 1u32;
        while remainder % number != 0 {
            quotientes.push(remainder * 10 / number);
            remainder = remainder * 10 % number;
            match remainderes.iter().position(|&x| x == remainder) {
                // 如果余数重复了，则说明找到循环了, 结束返回
                // 而不是商数(如 1/233 有两次商数为0)
                Some(index) => {
                    return quotientes[index..]
                        .iter()
                        .map(|num| std::char::from_digit(*num, 10).unwrap())
                        .collect();
                }
                None => remainderes.push(remainder),
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reciprocal_cycles_test() {
        assert_eq!(Solution::reciprocal_cycles(1), 1);
        assert_eq!(Solution::get_reciprocal_cycles(1), "".to_string());

        assert_eq!(Solution::reciprocal_cycles(6), 6);
        assert_eq!(Solution::get_reciprocal_cycles(6), "6".to_string());

        assert_eq!(Solution::reciprocal_cycles(7), 7);
        assert_eq!(Solution::reciprocal_cycles(10), 7);
        assert_eq!(Solution::get_reciprocal_cycles(7), "142857".to_string());

        assert_eq!(Solution::reciprocal_cycles(101), 97);
        assert_eq!(
            Solution::get_reciprocal_cycles(97),
            "010309278350515463917525773195876288659793814432989690721649484536082474226804123711340206185567".to_owned()
        );

        assert_eq!(Solution::reciprocal_cycles(1000), 983);
    }
}
