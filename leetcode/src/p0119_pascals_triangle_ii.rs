// 119. Pascal's Triangle II
// Easy

// Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle.

// Note that the row index starts from 0.


// In Pascal's triangle, each number is the sum of the two numbers directly above it.

// Example:

// Input: 3
// Output: [1,3,3,1]
// Follow up:

// Could you optimize your algorithm to use only O(k) extra space?

pub struct Solution {}


impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = vec![1; (row_index + 1) as usize];
        // line n = Cn1, Cn2, ... Cnn
        for i in 1..row_index {
            result[i as usize] = Solution::combina(row_index, i32::min(i, row_index-i));
        }
        result
    }
    fn combina(row_index: i32, x: i32) -> i32 {
        // TODO 计算Cnm, 目前采用比较暴力计算法, 会考虑内置的, 或者更加精确的
        if x == 0 || x == 1 { return row_index; }
        let mut result: f64 = 1.0;
        let mut m_max = row_index as f64;
        let mut n_min = x as f64;
        for _ in 1..=x {
            result *= m_max;
            m_max -= 1.0;
        }
        for _ in 1..=x {
            result /= n_min;
            n_min -= 1.0;
        }
        f64::ceil(result) as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_row_test() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
        assert_eq!(Solution::get_row(5), vec![1, 5, 10, 10, 5, 1]);
        assert_eq!(Solution::get_row(7), vec![1, 7, 21, 35, 35, 21, 7, 1]);
        assert_eq!(Solution::get_row(8), vec![1, 8, 28, 56, 70, 56, 28, 8, 1]);
        assert_eq!(Solution::get_row(13), vec![1,13,78,286,715,1287,1716,1716,1287,715,286,78,13,1]);
        assert_eq!(Solution::get_row(31), vec![1,31,465,4495,31465,169911,736281,2629575,7888725,20160075,44352165,84672315,141120525,206253075,265182525,300540195,300540195,265182525,206253075,141120525,84672315,44352165,20160075,7888725,2629575,736281,169911,31465,4495,465,31,1])
    }
}
