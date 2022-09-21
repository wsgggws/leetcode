// Pandigital products
// Problem 32
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.

// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.

// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.

// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.

pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn pandigital_products() -> u64 {
        let mut products: HashSet<u64> = HashSet::new();
        // TODO, 2000是试出来的，不知道如何求出它的上限
        for multiplicand in 1..2000 {
            for multiplier in 1..2000 {
                if Solution::is_pandigital_product(multiplicand, multiplier) {
                    products.insert(multiplicand * multiplier);
                }
            }
        }
        products.iter().sum()
    }

    fn is_pandigital_product(multiplicand: u64, multiplier: u64) -> bool {
        let product = multiplicand * multiplier;
        let strs: String = multiplicand.to_string() + &multiplier.to_string() + &product.to_string();
        if strs.len() != 9 {
            return false;
        }
        let digit_chars = strs.chars().collect::<HashSet<char>>();
        if digit_chars.contains(&'0') || digit_chars.len() != 9 {
            return false
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pandigital_products_test() {
        assert_eq!(Solution::pandigital_products(), 45228);
    }
}
