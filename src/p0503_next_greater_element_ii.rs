// 503. Next Greater Element II
// Medium

// Given a circular array (the next element of the last element is the first element of the array), print the Next Greater Number for every element. The Next Greater Number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, output -1 for this number.

// Example 1:
// Input: [1,2,1]
// Output: [2,-1,2]
// Explanation: The first 1's next greater number is 2;
// The number 2 can't find next greater number;
// The second 1's next greater number needs to search circularly, which is also 2.
// Note: The length of given array won't exceed 10000.

pub struct Solution {}

impl Solution {
    // 维护单调递减栈
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = vec![];
        let mut result: Vec<i32> = vec![-1; nums.len()];
        for i in 0..nums.len() * 2 {
            let num = nums[i % nums.len()];
            while stack.len() > 1 && nums[stack[stack.len() - 1]] < num {
                let pre_index = stack.pop().unwrap();
                result[pre_index] = num;
            }
            stack.push(i % nums.len());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_greater_elements_test() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
    }
}
