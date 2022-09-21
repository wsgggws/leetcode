// 1000-digit Fibonacci number
// Problem 25
// The Fibonacci sequence is defined by the recurrence relation:

// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// Hence the first 12 terms will be:

// F1 = 1
// F2 = 1
// F3 = 2
// F4 = 3
// F5 = 5
// F6 = 8
// F7 = 13
// F8 = 21
// F9 = 34
// F10 = 55
// F11 = 89
// F12 = 144
// The 12th term, F12, is the first term to contain three digits.

// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

pub struct Solution {}

struct Fib {
    x: (usize, String, String),
}

impl Fib {
    fn new() -> Fib {
        Fib { x: (0usize, "1".to_string(), "0".to_string()) }
    }

    fn add_strings(&self, str1: &String, str2: &String) -> String {
        let len1 = str1.len();
        let mut chars1: Vec<char> = str1.chars().rev().collect();
        let len2 = str2.len();
        let chars2: Vec<char> = str2.chars().rev().collect();
        for _ in 0..len2 - len1 {
            chars1.push('0')
        }
        let mut result = String::new();
        let mut carry = 0;
        for i in 0..len2 {
            let num = chars1[i].to_digit(10).unwrap() + chars2[i].to_digit(10).unwrap();
            result.push_str(&((num + carry) % 10).to_string());
            carry = (num + carry) / 10;
        }
        if carry > 0 {
            result.push_str(&carry.to_string());
        }
        result.chars()
            .rev()
            .collect()
    }
}

impl Iterator for Fib {
    type Item = (usize, String);
    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x.0 + 1, self.x.2.clone(), self.add_strings(&self.x.1, &self.x.2));
        Some((self.x.0, self.x.2.clone()))
    }
}

impl Solution {
    pub fn get_1000_digit_fibonacci_number(num: usize) -> usize {
        for (order, a_str) in Fib::new() {
            if a_str.len() >= num {
                return order;
            }
        }
        0
        // TODO 如何使用迭代器返回呢?
        // Fib::new().take_while(|(_, a_str)| a_str.len() >= num).next().unwrap().0
    }

    pub fn print_fibonacci_number(num: usize) {
        let mut iter = Fib::new();
        for _ in 0..num {
            println!("{:?}", iter.next().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_1000_digit_fibonacci_number_test() {
        assert_eq!(Solution::get_1000_digit_fibonacci_number(1), 1);
        assert_eq!(Solution::get_1000_digit_fibonacci_number(2), 7);
        assert_eq!(Solution::get_1000_digit_fibonacci_number(3), 12);
        assert_eq!(Solution::get_1000_digit_fibonacci_number(1000), 4782);
    }

    #[test]
    fn print_fibonacci_number_test() {
        assert_eq!(Solution::print_fibonacci_number(20), ());
    }
}
