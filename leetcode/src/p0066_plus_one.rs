// 66. Plus One
// Easy

// Given a non-empty array of digits representing a non-negative integer, plus one to the integer.

// The digits are stored such that the most significant digit is at the head of the list, and each element in the array contain a single digit.

// You may assume the integer does not contain any leading zero, except the number 0 itself.

// Example 1:

// Input: [1,2,3]
// Output: [1,2,4]
// Explanation: The array represents the integer 123.
// Example 2:

// Input: [4,3,2,1]
// Output: [4,3,2,2]
// Explanation: The array represents the integer 4321.

pub struct Solution {}


impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut num = 1;
        let len = digits.len();
        let mut result = Vec::new();
        let mut digits = digits;

        for _ in 0..len {
            if let Some(digit) = digits.pop() {
                let value = digit + num;
                if value > 9 {
                    num = 1;
                    result.insert(0, 0);
                }else{
                    num = 0;
                    result.insert(0, value);
                }
            }
        }
        if num == 1 { result.insert(0, 1); }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one_test() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
        assert_eq!(Solution::plus_one(vec![9, 0, 9]), vec![9, 1, 0]);
    }
}
