// 17. Letter Combinations of a Phone Number
// Medium

// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.

// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.



// Example:

// Input: "23"
// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
// Note:

// Although the above answer is in lexicographical order, your answer could be in any order you want.

pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let mut ans: Vec<String> = vec![];
        let mut cur_str = "".to_owned();
        Solution::dfs(&mut ans, &digits, &mut cur_str, 0);
        ans
    }

    fn dfs(ans: &mut Vec<String>, digits: &String, cur_str: &mut String, index: usize) {
        if index == digits.len() {
            ans.push(cur_str.clone());
            return;
        }
        for ch in Solution::get_letters(digits, index).chars() {
            cur_str.push(ch);
            Solution::dfs(ans, digits, &mut cur_str.clone(), index + 1);
            cur_str.pop();
        }
    }

    fn get_letters(digits: &String, index: usize) -> String {
        match digits.chars().nth(index) {
            Some('2') => "abc".to_owned(),
            Some('3') => "def".to_owned(),
            Some('4') => "ghi".to_owned(),
            Some('5') => "jkl".to_owned(),
            Some('6') => "mno".to_owned(),
            Some('7') => "pqrs".to_owned(),
            Some('8') => "tuv".to_owned(),
            Some('9') => "wxyz".to_owned(),
            Some(_) => panic!("error"),
            None => panic!("error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_combinations_test() {
        let empty: Vec<String> = vec![];
        assert_eq!(
            Solution::letter_combinations("".to_owned()),
            empty
        );
        assert_eq!(
            Solution::letter_combinations("23".to_owned()),
            vec![
                "ad".to_owned(),
                "ae".to_owned(),
                "af".to_owned(),
                "bd".to_owned(),
                "be".to_owned(),
                "bf".to_owned(),
                "cd".to_owned(),
                "ce".to_owned(),
                "cf".to_owned()
            ]
        );
        assert_eq!(
            Solution::letter_combinations("234".to_owned()),
            vec![
                "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi",
                "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei",
                "cfg", "cfh", "cfi"
            ]
        );
    }
}
