pub struct Solution {}


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let symbols = vec![
            ("MMM", 3, 3000),
            ("MM", 2, 2000),
            ("M", 1, 1000),
            ("CM", 2, 900),
            ("DCCC", 4, 800),
            ("DCC", 3, 700),
            ("DC", 2, 600),
            ("D", 1, 500),
            ("CD", 2, 400),
            ("CCC", 3, 300),
            ("CC", 2, 200),
            ("C", 1, 100),
            ("XC", 2, 90),
            ("LXXX", 4, 80),
            ("LXX", 3, 70),
            ("LX", 2, 60),
            ("L", 1, 50),
            ("XL", 2, 40),
            ("XXX", 3, 30),
            ("XX", 2, 20),
            ("X", 1, 10),
            ("IX", 2, 9),
            ("VIII", 4, 8),
            ("VII", 3, 7),
            ("VI", 2, 6),
            ("V", 1, 5),
            ("IV", 2, 4),
            ("III", 3, 3),
            ("II", 2, 2),
            ("I", 1, 1),
        ];
        let mut result = 0;
        let mut index = 0;
        for symbol in &symbols {
            if index+symbol.1 <= s.len() && symbol.0 == &s[index..(index+symbol.1)] {
                index += symbol.1;
                result += symbol.2;
                if index >= s.len() { break; }
            }
        }
        result
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn roman_to_int_test() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
