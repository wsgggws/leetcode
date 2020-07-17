pub struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if &num1[..] == "0" || &num2[..] == "0" {
            return "0".to_owned();
        }
        let ch_num1 = num1.chars().rev().collect::<Vec<char>>();
        let ch_num2 = num2.chars().rev().collect::<Vec<char>>();
        let mut ans: Vec<u32> = vec![0; num1.len() + num2.len()];
        for i in 0..num1.len() {
            for j in 0..num2.len() {
                let mul = ch_num1[i].to_digit(10).unwrap() * ch_num2[j].to_digit(10).unwrap();
                ans[i + j] += mul % 10;
                ans[i + j + 1] += mul / 10;
            }
        }
        for i in 0..ans.len() - 1 {
            ans[i + 1] += ans[i] / 10;
            ans[i] %= 10;
        }
        if ans[ans.len() - 1] == 0 {
            ans.pop();
        }
        ans.iter()
            .rev()
            .map(|&a| std::char::from_digit(a, 10).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_test() {
        assert_eq!(
            Solution::multiply("2".to_owned(), "3".to_owned()),
            "6".to_owned()
        );
        assert_eq!(
            Solution::multiply("0".to_owned(), "3".to_owned()),
            "0".to_owned()
        );
        assert_eq!(
            Solution::multiply("123".to_owned(), "456".to_owned()),
            "56088".to_owned()
        );
        assert_eq!(
            Solution::multiply("999".to_owned(), "999".to_owned()),
            "998001".to_owned()
        );
        assert_eq!(
            Solution::multiply("1000".to_owned(), "999".to_owned()),
            "999000".to_owned()
        );
    }
}
