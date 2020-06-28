// 303. Range Sum Query - Immutable
// Easy

// Given an integer array nums, find the sum of the elements between indices i and j (i ≤ j), inclusive.

// Example:
// Given nums = [-2, 0, 3, -5, 2, -1]

// sumRange(0, 2) -> 1
// sumRange(2, 5) -> -1
// sumRange(0, 5) -> -3
// Note:
// You may assume that the array does not change.
// There are many calls to sumRange function.

pub struct NumArray {
    pub sums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    pub fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sums[i+1] = sums[i] + nums[i];
        }
        Self { sums }
    }

    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sums[j as usize + 1] - self.sums[i as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_array_test() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3);
    }
}
