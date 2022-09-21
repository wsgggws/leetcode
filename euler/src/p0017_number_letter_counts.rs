// Number letter counts
// Problem 17
// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

pub struct Solution {}

impl Solution {
    pub fn number_letter_counts(n: usize) -> usize {
        (1..=n)
            .map(|num| Solution::english_number(num))
            .map(|strs| Solution::remove_space(&strs))
            .map(|strs| strs.len())
            .sum::<usize>()
    }

    fn english_number(n: usize) -> String {
        if n > 1000 {
            panic!("Plz set 0 <= n <= 1000");
        }

        let list0_9 = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        if n <= 9 {
            return list0_9[n].to_string();
        }
        if n <= 19 {
            let list = vec![
                "ten",
                "eleven",
                "twelve",
                "thirteen",
                "fourteen",
                "fifteen",
                "sixteen",
                "seventeen",
                "eighteen",
                "nineteen",
            ];
            return list[n - 10].to_string();
        }
        if n <= 99 {
            let tens: usize = n / 10; // 十位
            let units: usize = n % 10;
            let list = vec![
                "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty",
                "ninety",
            ];
            let str = list[tens].to_string();
            if units > 0 {
                return str + "-" + &Solution::english_number(units);
            }
            return str;
        }
        if n <= 999 {
            let hundreds: usize = n / 100; // 百位
            let others: usize = n % 100;
            let str = list0_9[hundreds].to_string() + " hundred";
            if others > 0 {
                return str + " and " + &Solution::english_number(others);
            }
            return str;
        }
        if n == 1000 {
            return "one thousand".to_string();
        }

        "".to_string()
    }

    fn remove_space(s: &str) -> String {
        // 是计算有多少个字母，而不是多少个单词
        s.chars()
            .filter(|c| *c != ' ' && *c != '-')
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_letter_counts_test() {
        assert_eq!(Solution::number_letter_counts(1), 3);
        assert_eq!(Solution::number_letter_counts(5), 19);
        assert_eq!(Solution::number_letter_counts(10), 39);
        assert_eq!(Solution::number_letter_counts(1000), 21124);
    }
}
