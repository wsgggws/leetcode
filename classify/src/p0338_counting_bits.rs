// 338. Counting Bits
// Medium

// Given a non negative integer number num. For every numbers i in the range 0 ≤ i ≤ num calculate the number of 1's in their binary representation and return them as an array.

// Example 1:

// Input: 2
// Output: [0,1,1]
// Example 2:

// Input: 5
// Output: [0,1,1,2,1,2]
// Follow up:

// It is very easy to come up with a solution with run time O(n*sizeof(integer)). But can you do it in linear time O(n) /possibly in a single pass?
// Space complexity should be O(n).
// Can you do it like a boss? Do it without using any builtin function like __builtin_popcount in c++ or in any other language.

pub struct Solution {}

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        (0..=num)
            .map(|n| n.count_ones() as i32)
            .collect()
    }

    pub fn count_bits_2(num: i32) -> Vec<i32> {
        // 动态规划
        let mut result = vec![0; num as usize + 1];
        for i in 1..=num as usize {
            result[i] = result[i & (i-1)] + 1;
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bits_test() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);

        assert_eq!(Solution::count_bits_2(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits_2(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
