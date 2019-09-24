pub struct Solution {}


impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let symbols = vec![
            ("I", 1),
            ("II", 2),
            ("III", 3),
            ("IV", 4),
            ("V", 5),
            ("VI", 6),
            ("VII", 7),
            ("VIII", 8),
            ("IX", 9),
            ("X", 10),
            ("XX", 20),
            ("XXX", 30),
            ("XL", 40),
            ("L", 50),
            ("LX", 60),
            ("LXX", 70),
            ("LXXX", 80),
            ("XC", 90),
            ("C", 100),
            ("CC", 200),
            ("CCC", 300),
            ("CD", 400),
            ("D", 500),
            ("DC", 600),
            ("DCC", 700),
            ("DCCC", 800),
            ("CM", 900),
            ("M", 1000),
            ("MM", 2000),
            ("MMM", 3000)
        ];
        let mut number = num;
        let mut mutilples = 1;
        let mut strs = Vec::new();
        for symbol in &symbols {
            let current_num = number % 10;
            if (current_num == 0) || (symbol.1 == current_num*mutilples) {
                number /= 10;
                mutilples *= 10;
                if current_num != 0 {
                    // strs.push(symbol.0);
                    strs.insert(0, symbol.0);
                }
            }
            if number <= 0 { break; }
        }
        // strs.reverse();
        let mut result = String::new();
        for a_str in &strs{
            result.push_str(a_str)
        }
        result
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn roman_to_int_test() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
        assert_eq!(Solution::int_to_roman(1000), "M".to_string());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX".to_string());
    }
}
