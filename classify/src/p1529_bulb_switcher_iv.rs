// 1529. Bulb Switcher IV
// Medium

// There is a room with n bulbs, numbered from 0 to n-1, arranged in a row from left to right. Initially all the bulbs are turned off.

// Your task is to obtain the configuration represented by target where target[i] is '1' if the i-th bulb is turned on and is '0' if it is turned off.

// You have a switch to flip the state of the bulb, a flip operation is defined as follows:

// Choose any bulb (index i) of your current configuration.
// Flip each bulb from index i to n-1.
// When any bulb is flipped it means that if it is 0 it changes to 1 and if it is 1 it changes to 0.

// Return the minimum number of flips required to form target.



// Example 1:

// Input: target = "10111"
// Output: 3
// Explanation: Initial configuration "00000".
// flip from the third bulb:  "00000" -> "00111"
// flip from the first bulb:  "00111" -> "11000"
// flip from the second bulb:  "11000" -> "10111"
// We need at least 3 flip operations to form target.
// Example 2:

// Input: target = "101"
// Output: 3
// Explanation: "000" -> "111" -> "100" -> "101".
// Example 3:

// Input: target = "00000"
// Output: 0
// Example 4:

// Input: target = "001011101"
// Output: 5


// Constraints:

// 1 <= target.length <= 10^5
// target[i] == '0' or target[i] == '1'

pub struct Solution {}

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut count: i32 = 0;
        let mut i = 0;
        let chs: Vec<char> = target.chars().collect();
        while i < target.len() && chs[i] == '0' { i += 1; }
        while i < target.len() {
            if chs[i] == '1' {
                while i < target.len() && chs[i] == '1' { i += 1; }
                count += 1;
            } else {
                while i < target.len() && chs[i] == '0' { i += 1; }
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_flips_test() {
        assert_eq!(Solution::min_flips("10111".to_owned()), 3);
        assert_eq!(Solution::min_flips("101".to_owned()), 3);
        assert_eq!(Solution::min_flips("00000".to_owned()), 0);
        assert_eq!(Solution::min_flips("001011101".to_owned()), 5);
    }
}
