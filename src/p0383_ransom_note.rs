// 383. Ransom Note
// Easy

// Given an arbitrary ransom note string and another string containing letters from all the magazines, write a function that will return true if the ransom note can be constructed from the magazines ; otherwise, it will return false.

// Each letter in the magazine string can only be used once in your ransom note.

// Note:
// You may assume that both strings contain only lowercase letters.

// canConstruct("a", "b") -> false
// canConstruct("aa", "ab") -> false
// canConstruct("aa", "aab") -> true

pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut ransom_note: Vec<char> = ransom_note.chars().collect();
        ransom_note.sort_by(|a, b| b.cmp(a));
        let mut magazine: Vec<char> = magazine.chars().collect();
        magazine.sort_by(|a, b| b.cmp(a));

        let mut index_i = 0_usize;
        let mut index_j = 0_usize;
        while index_j < magazine.len() && index_i < ransom_note.len() {
            if magazine[index_j] == ransom_note[index_i] {
                index_i += 1;
            }
            index_j += 1_usize;
        }
        if index_i == ransom_note.len() {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_test() {
        assert_eq!(
            Solution::can_construct("a".to_owned(), "b".to_owned()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_owned(), "ab".to_owned()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_owned(), "aab".to_owned()),
            true
        );
        assert_eq!(
            Solution::can_construct("aba".to_owned(), "aabaa".to_owned()),
            true
        );
        assert_eq!(
            Solution::can_construct(
                "bjaajgea".to_owned(),
                "affhiiicabhbdchbidghccijjbfjfhjeddgggbajhidhjchiedhdibgeaecffbbbefiabjdhggihccec"
                    .to_owned()
            ),
            true
        );
    }
}
