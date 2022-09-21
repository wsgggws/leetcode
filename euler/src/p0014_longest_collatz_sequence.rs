// Longest Collatz sequence
// Problem 14
// The following iterative sequence is defined for the set of positive integers:

// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:

// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

// Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

pub struct Solution {}

impl Solution {
    pub fn longest_collatz_sequence(n: u64) -> u64 {
        let mut maxs = 0;
        let mut result = 1;
        for num in 1..n {
            let c = Solution::collatz_len(num);
            if c > maxs {
                maxs = c;
                result = num;
            }
        }
        // 题目要求是哪个数字，而不是最大的长度
        result
    }

    fn collatz_len(num: u64) -> u64 {
        if num == 1u64 {
            return 1u64;
        }
        let y = if num % 2 == 0 { num / 2 } else { num * 3 + 1 };
        Solution::collatz_len(y) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_collatz_sequence_test() {
        assert_eq!(Solution::longest_collatz_sequence(13), 9);
        assert_eq!(Solution::longest_collatz_sequence(1000000), 837799);
    }
}
