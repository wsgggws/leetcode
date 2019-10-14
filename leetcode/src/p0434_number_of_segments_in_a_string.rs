// 434. Number of Segments in a String
// Easy

// Count the number of segments in a string, where a segment is defined to be a contiguous sequence of non-space characters.

// Please note that the string does not contain any non-printable characters.

// Example:

// Input: "Hello, my name is John"
// Output: 5

pub struct Solution {}


impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let v: Vec<&str> = s.trim().split_whitespace().collect();
        v.len() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_segments_test() {
        assert_eq!(Solution::count_segments("".to_owned()), 0);
        assert_eq!(Solution::count_segments("    ".to_owned()), 0);
        assert_eq!(Solution::count_segments("   abc d ".to_owned()), 2);
        assert_eq!(Solution::count_segments("Hello, my name is John".to_owned()), 5);
        assert_eq!(Solution::count_segments(", , , ,        a, eaefa".to_owned()), 6);

    }
}
