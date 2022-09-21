// Lexicographic permutations
// Problem 24
// A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

// 012   021   102   120   201   210

// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

use std::char;
use permutohedron::heap_recursive;

pub struct Solution {}

impl Solution {
    pub fn lexicographic_permutations(data: Vec<u32>, n: usize) -> String {
        if n <= 0 {
            panic!("Plz set n > 0");
        }
        let mut data = data;
        let mut permutations = Vec::new();
        heap_recursive(&mut data, |permutation| {
            permutations.push(permutation.to_vec())
        });
        // TODO 这里必须要进行排序，原因还需要看源代码
        permutations.sort();
        permutations[n-1].iter()
            .map(|&num| char::from_digit(num, 10).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexicographic_permutations_test() {
        assert_eq!(Solution::lexicographic_permutations(vec![0, 1, 2], 1), "012".to_string());
        assert_eq!(Solution::lexicographic_permutations(vec![0, 1, 2], 2), "021".to_string());
        assert_eq!(Solution::lexicographic_permutations(vec![0, 1, 2], 3), "102".to_string());
        assert_eq!(Solution::lexicographic_permutations(vec![0, 1, 2], 4), "120".to_string());
        assert_eq!(Solution::lexicographic_permutations(vec![0, 1, 2], 5), "201".to_string());
        assert_eq!(Solution::lexicographic_permutations(vec![0, 1, 2], 6), "210".to_string());
        assert_eq!(Solution::lexicographic_permutations(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1000000), "2783915460".to_string());
    }
}
