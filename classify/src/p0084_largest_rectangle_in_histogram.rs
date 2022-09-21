// 84. Largest Rectangle in Histogram
// Hard

// Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.

// Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].

// The largest rectangle is shown in the shaded area, which has area = 10 unit.

// Example:

// Input: [2,1,5,6,2,3]
// Output: 10

pub struct Solution {}

impl Solution {
    // 利用单调递增栈
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut new_heights: Vec<i32> = heights.clone();
        new_heights.push(0);

        let mut stack: Vec<usize> = vec![];
        let mut max_area = 0;
        for cur_index in 0..new_heights.len() {
            while stack.len() > 0 && new_heights[stack[stack.len() - 1]] >= new_heights[cur_index] {
                let pre_index = stack.pop().unwrap();
                let widths = if stack.is_empty() {
                    cur_index as i32
                } else {
                    (cur_index - stack[stack.len() - 1] - 1) as i32
                };
                max_area = i32::max(max_area, new_heights[pre_index] * widths);
            }
            stack.push(cur_index);
        }
        max_area
    }

    // 找局部峰值
    pub fn largest_rectangle_area_array(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        for i in 0..heights.len() {
            if i + 1 < heights.len() && heights[i] <= heights[i + 1] {
                continue;
            }
            let mut min_hight = heights[i];
            for j in (0..=i).rev() {
                min_hight = i32::min(min_hight, heights[j]);
                max_area = i32::max(max_area, min_hight * (i - j + 1) as i32);
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_rectangle_area_test() {
        assert_eq!(Solution::largest_rectangle_area(vec![1, 1]), 2);
        assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 2]), 4);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn largest_rectangle_area_arraytest() {
        assert_eq!(Solution::largest_rectangle_area_array(vec![1, 1]), 2);
        assert_eq!(Solution::largest_rectangle_area_array(vec![1, 2, 2]), 4);
        assert_eq!(
            Solution::largest_rectangle_area_array(vec![2, 1, 5, 6, 2, 3]),
            10
        );
    }
}
