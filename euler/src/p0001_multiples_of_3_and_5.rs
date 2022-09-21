// Multiples of 3 and 5
// Problem 1
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

pub struct Solution {}

impl Solution {
    pub fn multiples_of_3_and_5() -> u32 {
        (1..1000)
            .filter(|x| x%3 == 0 || x%5 == 0)
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiples_of_3_and_5_test() {
        assert_eq!(Solution::multiples_of_3_and_5(), 233168);
    }
}
