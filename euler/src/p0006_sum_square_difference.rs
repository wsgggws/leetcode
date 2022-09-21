// Sum square difference
// Problem 6
// The sum of the squares of the first ten natural numbers is,

// 12+22+...+102=385
// 12+22+...+102=385
// The square of the sum of the first ten natural numbers is,

// (1+2+...+10)2=552=3025
// (1+2+...+10)2=552=3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025−385=26403025−385=2640.

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

pub struct Solution {}

impl Solution {
    pub fn sum_square_difference() -> u64 {
        (1..=100u64).sum::<u64>().pow(2) - (1..=100u64).map(|num| num * num).sum::<u64>()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_square_difference_test() {
        assert_eq!(Solution::sum_square_difference(), 25164150);
    }
}
