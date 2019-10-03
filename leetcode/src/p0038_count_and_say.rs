// 38. Count and Say
// Easy

// The count-and-say sequence is the sequence of integers with the first five terms as following:

// 1.     1
// 2.     11
// 3.     21
// 4.     1211
// 5.     111221
// 1 is read off as "one 1" or 11.
// 11 is read off as "two 1s" or 21.
// 21 is read off as "one 2, then one 1" or 1211.

// Given an integer n where 1 ≤ n ≤ 30, generate the nth term of the count-and-say sequence.

// Note: Each term of the sequence of integers will be represented as a string.


// Example 1:

// Input: 1
// Output: "1"
// Example 2:

// Input: 4
// Output: "1211"

pub struct Solution {}


use std::char::from_digit;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = vec!['1'];
        for _ in 0..n-1 {
            let mut part = Vec::new();
            let mut i = 0_usize;
            while i < result.len() {
                let mut j = i + 1;
                while j < result.len() && result[j] == result[i] {
                    j += 1;
                }
                // unwrap 或expect 直接将正确值取出来
                part.push(from_digit((j - i) as u32, 10).unwrap());
                part.push(result[i]);
                i = j;
            }
            result = part;
        }
        result.iter().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_and_say_test() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
        assert_eq!(Solution::count_and_say(5), "111221".to_string());
        assert_eq!(Solution::count_and_say(6), "312211".to_string());
    }
}
