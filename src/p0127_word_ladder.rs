// 127. Word Ladder
// Medium

// Given two words (beginWord and endWord), and a dictionary's word list, find the length of shortest transformation sequence from beginWord to endWord, such that:

// Only one letter can be changed at a time.
// Each transformed word must exist in the word list.
// Note:

// Return 0 if there is no such transformation sequence.
// All words have the same length.
// All words contain only lowercase alphabetic characters.
// You may assume no duplicates in the word list.
// You may assume beginWord and endWord are non-empty and are not the same.
// Example 1:

// Input:
// beginWord = "hit",
// endWord = "cog",
// wordList = ["hot","dot","dog","lot","log","cog"]

// Output: 5

// Explanation: As one shortest transformation is "hit" -> "hot" -> "dot" -> "dog" -> "cog",
// return its length 5.
// Example 2:

// Input:
// beginWord = "hit"
// endWord = "cog"
// wordList = ["hot","dot","dog","lot","log"]

// Output: 0

// Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.

pub struct Solution {}

use std::collections::{ HashSet, VecDeque };
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {

        let word_set : HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return 0
        }

        let mut visit_set : HashSet<String> = HashSet::new();
        visit_set.insert(begin_word.clone());
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((begin_word.clone(), 1));

        while !queue.is_empty() {
            let (a_str, steps) = queue.pop_front().unwrap();
            for a_new_str in Solution::get_a_to_z_str(&a_str).iter() {
                if !visit_set.contains(a_new_str) && word_set.contains(a_new_str) {
                    if a_new_str == &end_word[..] {
                        return steps + 1;
                    }
                    if a_new_str == &begin_word[..] {
                        visit_set.insert(a_new_str.to_string());
                        continue;
                    }
                    queue.push_back((a_new_str.to_string(), steps + 1));
                    visit_set.insert(a_new_str.to_string());
                }
            }
        }
        0
    }
    fn get_a_to_z_str(a_str: &str) -> Vec<String> {
        let mut ans: Vec<String> = vec![];
        for i in 0..a_str.len() {
            let a_copy_str = a_str.clone();
            for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
                if i == 0 {
                    ans.push(ch.to_string() + &a_str[1..]);
                } else if i == a_str.len() - 1 {
                    ans.push(a_copy_str[..i].to_owned() + &ch.to_string());
                } else {
                    ans.push(a_copy_str[0..i].to_owned() + &ch.to_string() + &a_copy_str[i+1..])
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ladder_length_test() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                    "cog".to_owned()
                ]
            ),
            5
        );
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                ]
            ),
            0
        );
        assert_eq!(
            Solution::ladder_length(
                "leet".to_owned(),
                "code".to_owned(),
                vec![
                    "lest".to_owned(),
                    "leet".to_owned(),
                    "lose".to_owned(),
                    "code".to_owned(),
                    "lode".to_owned(),
                    "robe".to_owned(),
                    "lost".to_owned()
                ]
            ),
            6
        );
    }
}
