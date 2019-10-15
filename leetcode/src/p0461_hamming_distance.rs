// 461. Hamming Distance
// Easy

// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

// Given two integers x and y, calculate the Hamming distance.

// Note:
// 0 ≤ x, y < 231.

// Example:

// Input: x = 1, y = 4

// Output: 2

// Explanation:
// 1   (0 0 0 1)
// 4   (0 1 0 0)
//        ↑   ↑

// The above arrows point to positions where the corresponding bits are different.

pub struct Solution {}


impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut xy = x ^ y;
        let mut result = 0;
        while xy > 0 {
            if xy % 2 == 1 { result += 1; }
            xy /= 2;
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance_test() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }
}
