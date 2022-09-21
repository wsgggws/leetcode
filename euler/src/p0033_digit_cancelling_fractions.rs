// Digit cancelling fractions
// Problem 33
// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.

// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.

// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

pub struct Solution {}

impl Solution {
    pub fn digit_cancelling_fractions() -> u32 {
        let (mut molecular_product, mut denominator_product) = (1u32, 1u32);
        for molecular in 10..100 {
            for denominator in molecular+1..100 {
                if Solution::is_digit_cancelling_fraction(molecular, denominator) {
                    molecular_product *= molecular;
                    denominator_product *= denominator;
                }
            }
        }
        denominator_product / Solution::gcd(molecular_product, denominator_product)
    }

    fn is_digit_cancelling_fraction(molecular: u32, denominator: u32) -> bool {
        let gcd_value = Solution::gcd(molecular, denominator);
        if molecular % 10 == molecular / 10 || denominator % 10 == denominator / 10 {
            return false;
        }
        (molecular / gcd_value) * (denominator % 10) == (denominator / gcd_value) * (molecular / 10) && molecular % 10 == denominator / 10
    }

    fn gcd(molecular: u32, denominator: u32) -> u32 {
        if denominator == 0 {molecular} else {Solution::gcd(denominator , molecular % denominator)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_cancelling_fractions_test() {
        assert_eq!(Solution::digit_cancelling_fractions(), 100);
    }
}
