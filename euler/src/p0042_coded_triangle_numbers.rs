// Coded triangle numbers
// Problem 42
// The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten triangle numbers are:

// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

// By converting each letter in a word to a number corresponding to its alphabetical position and adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55 = t10. If the word value is a triangle number then we shall call the word a triangle word.

// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?

pub struct Solution {}

use std::fs;
use std::collections::HashSet;
impl Solution {
    pub fn coded_triangle_numbers() -> usize {
        let data_str = fs::read_to_string("data/p0042_words.txt")
            .expect("read file p0042_words.txt raise a error");
        let data = Solution::remove_quote_and_line_breaked(&data_str);
        let words: Vec<&str> = data.split(",").collect();
        let set_table = (1..100).map(|x| x * (x+1) / 2).collect::<HashSet<u32>>();
        words.iter()
            .map(|word| Solution::get_word_value(word))
            .filter(|&value| set_table.contains(&value))
            .count()
    }


    fn remove_quote_and_line_breaked(data: &str) -> String {
        data.chars()
            .filter(|c| *c != '"' && *c != '\n')
            .collect()
    }

    fn get_word_value(word: &str) -> u32 {
        word.chars()
            .map(|x| (x as u8 - 'A' as u8) as u32 + 1)
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coded_triangle_numbers_test() {
        assert_eq!(Solution::coded_triangle_numbers(), 162);
    }
    #[test]
    fn get_word_value_test() {
        assert_eq!(Solution::get_word_value(&"A"), 1);
        assert_eq!(Solution::get_word_value(&"SKY"), 55);
    }
}
