// Largest palindrome product
// Problem 4
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.
//

pub struct Solution {}

impl Solution {
    pub fn largest_palindrame_product() -> u64 {
        let mut result = 0u64;
        for a in (900u64..=999u64).rev(){
            for b in (900u64..=999u64).rev() {
                if Solution::is_palindrae_product(a * b){
                    result = u64::max(result, a * b);
                }
            }
        }
        result
    }

    fn is_palindrae_product(num: u64) -> bool {
        num.to_string().chars().eq(num.to_string().chars().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_palindrame_product_test() {
        assert_eq!(Solution::largest_palindrame_product(), 906609);
    }
}
