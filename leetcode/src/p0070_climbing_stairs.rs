// 70. Climbing Stairs
// Easy

// You are climbing a stair case. It takes n steps to reach to the top.

// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

// Note: Given n will be a positive integer.

// Example 1:

// Input: 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps
// Example 2:

// Input: 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step

pub struct Solution {}


impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // // 如果使用递归, 会Time Limit Exceeded
        // // f[n] = f[n-1] + f[n-2];
        // if n == 1 { return 1; }
        // else if n == 2 { return 2; }
        // else {
        //     return Solution::climb_stairs(n-1) + Solution::climb_stairs(n-2);
        // }

        if n == 1 { return 1; }
        if n == 2 { return 2; }
        let mut before_two = 1;
        let mut before_one = 2;
        let mut result = 0;
        for _ in 2..n {
            result = before_two + before_one;
            before_two = before_one;
            before_one = result;
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climb_stairs_test() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
