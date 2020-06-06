// 441. Arranging Coins
// Easy

// You have a total of n coins that you want to form in a staircase shape, where every k-th row must have exactly k coins.

// Given n, find the total number of full staircase rows that can be formed.

// n is a non-negative integer and fits within the range of a 32-bit signed integer.

// Example 1:

// n = 5

// The coins can form the following rows:
// ¤
// ¤ ¤
// ¤ ¤

// Because the 3rd row is incomplete, we return 2.
// Example 2:

// n = 8

// The coins can form the following rows:
// ¤
// ¤ ¤
// ¤ ¤ ¤
// ¤ ¤

// Because the 4th row is incomplete, we return 3.

pub struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }
        let mut start = 0_i64;
        let mut end = n as i64;
        while start <= end {
            let mid: i64 = (start + end) / 2_i64;
            if mid * (mid + 1) > n as i64 * 2 {
                end = mid - 1_i64;
            } else if mid * (mid + 1) == n as i64 * 2 {
                return mid as i32;
            } else if mid * (mid + 1) < n as i64 * 2 && n as i64 * 2 <= (mid - 1) * mid {
                return (mid - 1) as i32;
            } else {
                start = mid + 1_i64;
            }
        }
        end as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrange_coins_test() {
        assert_eq!(Solution::arrange_coins(0), 0);
        assert_eq!(Solution::arrange_coins(1), 1);
        assert_eq!(Solution::arrange_coins(2), 1);
        assert_eq!(Solution::arrange_coins(3), 2);
        assert_eq!(Solution::arrange_coins(4), 2);
        assert_eq!(Solution::arrange_coins(5), 2);
        assert_eq!(Solution::arrange_coins(6), 3);
        assert_eq!(Solution::arrange_coins(7), 3);
        assert_eq!(Solution::arrange_coins(8), 3);
        assert_eq!(Solution::arrange_coins(9), 3);
        assert_eq!(Solution::arrange_coins(10), 4);
    }
}
