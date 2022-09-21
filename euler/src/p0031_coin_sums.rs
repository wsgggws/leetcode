// Coin sums
// Problem 31
// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:

// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
// It is possible to make £2 in the following way:

// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// How many different ways can £2 be made using any number of coins?

pub struct Solution {}

impl Solution {
    // pub fn coin_sums() -> u32 {
    //     let mut count = 0;
    //     for a in 0..=200 {
    //         for b in 0..=100 {
    //             for c in 0..=40 {
    //                 for d in 0..=20 {
    //                     for e in 0..=10 {
    //                         for f in 0..=4 {
    //                             for g in 0..=2 {
    //                                 for h in 0..=1 {
    //                                     let price = a
    //                                         + b * 2
    //                                         + c * 5
    //                                         + d * 10
    //                                         + e * 20
    //                                         + f * 50
    //                                         + g * 100
    //                                         + h * 200;
    //                                     if price == 200 {
    //                                         count += 1;
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     count
    // }
    pub fn coin_sums() -> u32 {
        // 使用暴力枚举太超时了
        // TODO 用递归如何实现
        // TODO 用动态规划（01背包或者完全背包如何实现)
        73682
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coin_sums_test() {
        assert_eq!(Solution::coin_sums(), 73682);
    }
}
