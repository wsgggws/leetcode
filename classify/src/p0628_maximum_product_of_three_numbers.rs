// 628. Maximum Product of Three Numbers
// Easy

// Given an integer array, find three numbers whose product is maximum and output the maximum product.

// Example 1:

// Input: [1,2,3]
// Output: 6

// Example 2:

// Input: [1,2,3,4]
// Output: 24

// Note:

// The length of the given array will be in range [3,104] and all elements are in the range [-1000, 1000].
// Multiplication of any three numbers in the input won't exceed the range of 32-bit signed integer.

pub struct Solution {}

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut max1 = i32::min_value();
        let mut max2 = i32::min_value();
        let mut max3 = i32::min_value();
        let mut min1 = i32::max_value();
        let mut min2 = i32::max_value();

        // 最大三个数的乘积要么为三个最大值, 或者一个最大值与两个最小值的乘积.
        for num in nums.into_iter() {
            if num > max1 {
                max3 = max2;
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max3 = max2;
                max2 = num;
            } else if num > max3 {
                max3 = num;
            }

            if num < min1 {
                min2 = min1;
                min1 = num;
            } else if num < min2 {
                min2 = num;
            }
        }

        i32::max(max1 * max2 * max3, max1 * min1 * min2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_product_test() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
        assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
        assert_eq!(Solution::maximum_product(vec![1, -2, 3, 4]), 12);
        assert_eq!(Solution::maximum_product(vec![1, -2, 3, -4]), 24);
        assert_eq!(Solution::maximum_product(vec![-1, 2, 3]), -6);
    }
}
