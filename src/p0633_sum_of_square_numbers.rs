// Given a non-negative integer c, your task is to decide whether there're two integers a and b such that a2 + b2 = c.

// Example 1:

// Input: 5
// Output: True
// Explanation: 1 * 1 + 2 * 2 = 5

// Example 2:

// Input: 3
// Output: False

pub struct Solution {}

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        // 0 也是整数
        let (mut i, mut j) = (0, (c as f32).sqrt() as i32);
        while i <= j {
            let sum = i * i + j * j;
            if sum == c {
                return true;
            } else if sum > c {
                j -= 1;
            } else {
                i += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn judge_square_sum_test() {
        assert_eq!(Solution::judge_square_sum(1), true);
        assert_eq!(Solution::judge_square_sum(5), true);
        assert_eq!(Solution::judge_square_sum(8), true);
        assert_eq!(Solution::judge_square_sum(7), false);
    }
}
