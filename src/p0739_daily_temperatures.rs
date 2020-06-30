// 739. Daily Temperatures
// Medium

// Given a list of daily temperatures T, return a list such that, for each day in the input, tells you how many days you would have to wait until a warmer temperature. If there is no future day for which this is possible, put 0 instead.

// For example, given the list of temperatures T = [73, 74, 75, 71, 69, 72, 76, 73], your output should be [1, 1, 4, 2, 1, 1, 0, 0].

// Note: The length of temperatures will be in the range [1, 30000]. Each temperature will be an integer in the range [30, 100].

pub struct Solution {}

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![1; t.len()];
        result[t.len() - 1] = 0;
        for i in (0..t.len()-1).rev() {
            let mut j = i;
            while j < t.len() {
                if t[i] < t[j] {
                    result[i] = (j - i) as i32;
                    break;
                } else if result[j] == 0 {
                    result[i] = 0;
                    break;
                }
                j += result[j] as usize;
            }
        }
        result
    }

    // 题目意思确实不太好懂
    // 比73高的温度74，1
    // 比74高的温度75, 1
    // 比75高的温度为76， 6-2 = 4
    pub fn daily_temperatures_stack(t: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; t.len()];
        let mut stack: Vec<usize> = vec![];
        for current_index in 0..t.len() {
            while stack.len() > 0 && t[current_index] > t[stack[stack.len() - 1]] {
                let pre_index = stack.pop().unwrap();
                result[pre_index] = (current_index - pre_index) as i32;
            }
            stack.push(current_index);
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_temperatures_test() {
        assert_eq!(Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn daily_temperatures_stack_test() {
        assert_eq!(Solution::daily_temperatures_stack(vec![73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }
}
