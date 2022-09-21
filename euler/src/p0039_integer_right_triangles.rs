// Integer right triangles
// Problem 39
// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

// {20,48,52}, {24,45,51}, {30,40,50}

// For which value of p ≤ 1000, is the number of solutions maximised?

pub struct Solution {}

impl Solution {
    pub fn integer_right_triangles(n: u64) -> u64 {
        (12..=n)
            .map(|x| Solution::get_triangles(x))
            .max_by_key(|a| a.1)
            .unwrap()
            .0
    }

    fn get_triangles(number: u64) -> (u64, u64) {
        let mut count = 0;
        for a in 2..=number/2 {
            for b in a..=number/2 {
                let c = number - a - b;
                if a * a + b * b == c * c {
                    count += 1;
                }
            }
        }
        (number, count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_right_triangles_test() {
        assert_eq!(Solution::integer_right_triangles(12), 12);
        assert_eq!(Solution::integer_right_triangles(120), 120);
        assert_eq!(Solution::integer_right_triangles(1000), 840);
    }

    #[test]
    fn get_triangles_test() {
        // a: 3, b: 4, c: 5
        assert_eq!(Solution::get_triangles(12), (12, 1));

        // {20,48,52}, {24,45,51}, {30,40,50}
        assert_eq!(Solution::get_triangles(120), (120, 3));

        // a: 40, b: 399, c: 401
        // a: 56, b: 390, c: 394
        // a: 105, b: 360, c: 375
        // a: 120, b: 350, c: 370
        // a: 140, b: 336, c: 364
        // a: 168, b: 315, c: 357
        // a: 210, b: 280, c: 350
        // a: 240, b: 252, c: 348
        assert_eq!(Solution::get_triangles(840), (840, 8));
    }
}
