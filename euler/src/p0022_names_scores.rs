// Names scores
// Problem 22
// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.

// What is the total of all the name scores in the file?

pub struct Solution {}

use std::fs;
impl Solution {
    pub fn get_names_scores() -> u64 {
        let data_str = fs::read_to_string("data/p0022_names.txt")
            .expect("read file p022_names.txt raise a error");
        let data = Solution::remove_quote_and_line_breaked(&data_str);
        let mut names: Vec<&str> = data.split(",").collect();
        names.sort();

        names.iter()
            .enumerate()
            .map(|(index, name)| (index + 1) as u64 * Solution::get_word_score(name))
            .sum::<u64>()
    }

    fn remove_quote_and_line_breaked(data: &str) -> String {
        data.chars()
            .filter(|c| *c != '"' && *c != '\n')
            .collect()
    }

    fn letter_number(ch: char) -> u64 {
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        letters.chars()
            .position(|c| c == ch)
            .unwrap() as u64 + 1
    }

    fn get_word_score(word: &str) -> u64 {
        word.chars()
            .map(|ch| Solution::letter_number(ch))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_names_scores_test() {
        assert_eq!(Solution::get_names_scores(), 871198282);
    }
}
