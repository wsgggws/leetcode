// Quadratic primes
// Problem 27
// Euler discovered the remarkable quadratic formula:

// n2+n+41
// It turns out that the formula will produce 40 primes for the consecutive integer values 0≤n≤39. However, when n=40,402+40+41=40(40+1)+41 is divisible by 41, and certainly when n=41,412+41+41 is clearly divisible by 41.

// The incredible formula n2−79n+1601 was discovered, which produces 80 primes for the consecutive values 0≤n≤79. The product of the coefficients, −79 and 1601, is −126479.

// Considering quadratics of the form:

// n2+an+b, where |a|<1000 and |b|≤1000

// where |n| is the modulus/absolute value of n
// e.g. |11|=11 and |−4|=4
// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n=0.

pub struct Solution {}

impl Solution {
    pub fn quadratic_primes() -> i64 {
        let mut max_prime_len = 0; 
        let mut result = 0;
        for a in -999..=999 {
            for b in -1000..=1000 {
                let prime_series_len = Solution::consecutive_primes(a, b);
                if prime_series_len > max_prime_len {
                    max_prime_len = prime_series_len; 
                    // println!("primes:{}, a: {}, b: {}, a*b: {}", prime_series_len, a, b, a * b );
                    result = a * b;
                } 
            }
        }
        result
    }

    fn is_prime(num: u64) -> bool {
        if num == 2 || num == 3 {
            return true;
        }
        !(2..=(num as f64).sqrt().ceil() as u64)
            .any(|value| num % value == 0)
    }

    fn consecutive_primes(a: i64, b: i64) -> i64 {
        for n in 0..1000 {
            let y: i64 = n * n + a * n + b;
            if y < 0 || !Solution::is_prime(y as u64) {
                return n as i64;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_primes_test() {
        // primes:1, a: -999, b: 0, a*b: 0
        // primes:2, a: -998, b: 997, a*b: -995006
        // primes:3, a: -499, b: 997, a*b: -497503
        // primes:4, a: -325, b: 977, a*b: -317525
        // primes:5, a: -245, b: 977, a*b: -239365
        // primes:6, a: -199, b: 971, a*b: -193229
        // primes:7, a: -163, b: 983, a*b: -160229
        // primes:8, a: -131, b: 941, a*b: -123271
        // primes:9, a: -121, b: 947, a*b: -114587
        // primes:11, a: -105, b: 967, a*b: -101535
        // primes:71, a: -61, b: 971, a*b: -59231
        assert_eq!(Solution::quadratic_primes(), -59231);
    }
}
