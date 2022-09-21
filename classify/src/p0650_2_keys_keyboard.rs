// 650. 2 Keys Keyboard
// Medium

// Initially on a notepad only one character 'A' is present. You can perform two operations on this notepad for each step:

// Copy All: You can copy all the characters present on the notepad (partial copy is not allowed).
// Paste: You can paste the characters which are copied last time.

// Given a number n. You have to get exactly n 'A' on the notepad by performing the minimum number of steps permitted. Output the minimum number of steps to get n 'A'.

// Example 1:

// Input: 3
// Output: 3
// Explanation:
// Intitally, we have one character 'A'.
// In step 1, we use Copy All operation.
// In step 2, we use Paste operation to get 'AA'.
// In step 3, we use Paste operation to get 'AAA'.

// Note:

// The n will be in the range [1, 1000].

pub struct Solution {}

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut result = 0;
        let mut copy_n = n;
        for i in 2..=n {
            while copy_n % i == 0 {
                result += i;
                copy_n /= i;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_steps_test() {
        assert_eq!(Solution::min_steps(3), 3);
        assert_eq!(Solution::min_steps(4), 4);
    }
}
