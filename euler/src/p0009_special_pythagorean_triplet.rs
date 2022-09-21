// Special Pythagorean triplet
// Problem 9
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

// a2 + b2 = c2
// For example, 32 + 42 = 9 + 16 = 25 = 52.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

pub struct Solution {}

impl Solution {
    pub fn special_pythagorean_triplet() -> i32 {
        for a in 2..1000 {
            for b in a+1..1000 {
                let c = 1000 -  a -  b;
                if a * a + b * b == c * c {
                    return a * b * c;
                }
            }
        }
        0
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_pythagorean_triplet_test() {
        assert_eq!(Solution::special_pythagorean_triplet(), 31875000);
    }
}
