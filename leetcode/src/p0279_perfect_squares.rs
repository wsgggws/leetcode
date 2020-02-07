// 279. Perfect Squares
// Medium

// Given a positive integer n, find the least number of perfect square numbers (for example, 1, 4, 9, 16, ...) which sum to n.

// Example 1:

// Input: n = 12
// Output: 3 
// Explanation: 12 = 4 + 4 + 4.
// Example 2:

// Input: n = 13
// Output: 2
// Explanation: 13 = 4 + 9.

pub struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let squares: Vec<i32> = Solution::get_squares(n);
        let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
        for i in 1..=n {
            let mut mins = i32::max_value();
            for &square in squares.iter() {
                if square > i {
                    break
                }
                mins = i32::min(mins, dp[(i-square) as usize] + 1);
            }
            dp[i as usize] = mins
        }
        dp[n as usize]
    }

    fn get_squares(n: i32) -> Vec<i32> {
        let mut squares: Vec<i32> = Vec::new();
        let mut diff = 3;
        let mut square = 1;
        while square <= n {
            squares.push(square);
            square += diff;
            diff += 2;
        }
        squares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_squares_test() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
