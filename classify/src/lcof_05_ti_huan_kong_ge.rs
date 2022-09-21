// 剑指 Offer 05. 替换空格
// 请实现一个函数，把字符串 s 中的每个空格替换成"%20"。

 

// 示例 1：

// 输入：s = "We are happy."
// 输出："We%20are%20happy."
 

// 限制：

// 0 <= s 的长度 <= 10000

pub struct Solution {}

impl Solution {
    pub fn replace_space(s: String) -> String {
        if s.len() == 0 {
            return "".to_owned();
        }
        let space_count = s.chars()
            .filter(|&ch| ch==' ')
            .collect::<Vec<char>>()
            .len();

        let mut tmp: String = s.clone();
        // 原先有一个‘ ’空间, 故只要申请2位空格数量空间
        for _ in 0..2 * space_count {
            tmp.push(' ');
        }
        // 设定有足够的空间原地修改
        let mut ans = tmp.chars().collect::<Vec<char>>();

        let mut j = ans.len() - 1;
        for i in (0..s.len()).rev() {
            if ans[i] == ' ' {
                ans[j] = '0';
                ans[j-1] = '2';
                ans[j-2] = '%';
                if j >= 3 { j -= 3 };
            } else {
                ans.swap(i, j);
                if j >= 1 { j -= 1 };
            }
        }

        ans.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_space_test() {
        assert_eq!(Solution::replace_space("We are happy.".to_string()), "We%20are%20happy.".to_owned());
        assert_eq!(Solution::replace_space("  ".to_string()), "%20%20".to_owned());
        assert_eq!(Solution::replace_space(" . ".to_string()), "%20.%20".to_owned());
        assert_eq!(Solution::replace_space(".".to_string()), ".".to_owned());
    }
}
