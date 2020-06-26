pub struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if c == ']' {

                // 取[]里面的所有字符
                let mut tmp: Vec<char> = vec![];
                while let Some(ch) = stack.pop() {
                    if ch == '[' {
                        break
                    } else {
                        tmp.push(ch);
                    }
                }

                // 取数字
                let mut nums: Vec<char> = vec![];
                while let Some(ch) = stack.pop() {
                    if '0' <= ch && ch <= '9' {
                        nums.push(ch);
                    } else {
                        stack.push(ch);
                        break;
                    }
                }

                // 重新推入栈
                let number = nums.iter().rev().collect::<String>().parse::<i32>().unwrap();
                for _ in 0..number {
                    for t_c in tmp.iter().rev() {
                        stack.push(*t_c);
                    }
                }
            } else {
                stack.push(c)
            }
        }
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_string_test() {
        assert_eq!(Solution::decode_string("3[a]2[bc]".to_owned()), "aaabcbc".to_owned());
        assert_eq!(Solution::decode_string("3[a2[c]]".to_owned()), "accaccacc".to_owned());
        assert_eq!(Solution::decode_string("2[abc]3[cd]ef".to_owned()), "abcabccdcdcdef".to_owned());
        assert_eq!(Solution::decode_string("abc3[cd]xyz".to_owned()), "abccdcdcdxyz".to_owned());
        assert_eq!(Solution::decode_string("10[le]".to_owned()), "lelelelelelelelelele".to_owned());
    }
}
