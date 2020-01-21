// 744. Find Smallest Letter Greater Than Target
// Easy

// Given a list of sorted characters letters containing only lowercase letters, and given a target letter target, find the smallest element in the list that is larger than the given target.

// Letters also wrap around. For example, if the target is target = 'z' and letters = ['a', 'b'], the answer is 'a'.

// Examples:
// Input:
// letters = ["c", "f", "j"]
// target = "a"
// Output: "c"

// Input:
// letters = ["c", "f", "j"]
// target = "c"
// Output: "f"

// Input:
// letters = ["c", "f", "j"]
// target = "d"
// Output: "f"

// Input:
// letters = ["c", "f", "j"]
// target = "g"
// Output: "j"

// Input:
// letters = ["c", "f", "j"]
// target = "j"
// Output: "c"

// Input:
// letters = ["c", "f", "j"]
// target = "k"
// Output: "c"
// Note:
// letters has a length in range [2, 10000].
// letters consists of lowercase letters, and contains at least 2 unique letters.
// target is a lowercase letter.

pub struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut left, mut right) = (0, letters.len() as i32 -1);
        while left <= right {
            let mid = left + (right - left) / 2;
            if letters[mid as usize] <= target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        if left >= letters.len() as i32 { letters[0_usize] } else { letters[left as usize] }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn next_greatest_letter_test() {
        assert_eq!(Solution::next_greatest_letter(vec!['a', 'b'], 'z'), 'a');
        assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
        assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
        assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd'), 'f');
        assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'g'), 'j');
        assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'j'), 'c');
    }
}
