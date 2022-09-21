// Number spiral diagonals
// Problem 28
// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:

// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13

// It can be verified that the sum of the numbers on the diagonals is 101.

// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

pub struct Solution {}


impl Solution {
    pub fn number_spiral_diagonals(n: usize) -> u64 {
        if n % 2 == 0 {
            panic!("Plz set n to a odd!");
        }
        let mut result = 1u64;
        for counts in (3..=n as u64).step_by(2) {
            result += ((counts-2)*(counts-2)+(counts-1)..)
                .step_by((counts-1) as usize)
                .take(4)
                .sum::<u64>();
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_spiral_diagonals_test() {
        assert_eq!(Solution::number_spiral_diagonals(3), 25);
        assert_eq!(Solution::number_spiral_diagonals(5), 101);
        assert_eq!(Solution::number_spiral_diagonals(1001), 669171001);
    }
}
