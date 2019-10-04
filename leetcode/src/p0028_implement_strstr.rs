// 28. Implement strStr()
// Easy

// Implement strStr().

// Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:

// Input: haystack = "hello", needle = "ll"
// Output: 2
// Example 2:

// Input: haystack = "aaaaa", needle = "bba"
// Output: -1
// Clarification:

// What should we return when needle is an empty string? This is a great question to ask during an interview.

// For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().

pub struct Solution {}


impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // 自己实现, 能通过
        // if needle.is_empty() { return 0; }
        // let haystack_lens = haystack.len();
        // let needle_lens = needle.len();

        // let mut i = 0_usize;
        // while i <= haystack_lens - needle_lens {
        //     if &haystack[i..i+needle_lens] == &needle[..] {
        //         return i as i32;
        //     }
        //     i += 1;
        // }
        // -1_i32

        // 使用内置的函数库, 速度一致, 内存一致
        if needle.is_empty() { return 0; }
        haystack.find(&needle).map_or(-1_i32, |v| v as i32)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_str_test() {
        assert_eq!(Solution::str_str("hello".to_string(), "".to_string()), 0);
        assert_eq!(Solution::str_str("hello".to_string(), "llohel".to_string()), -1);
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(Solution::str_str("hello".to_string(), "lo".to_string()), 3);
        assert_eq!(Solution::str_str("aaaaa".to_string(), "bba".to_string()), -1);
    }
}
