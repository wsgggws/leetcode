// 22. Generate Parentheses
// Medium

// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

// For example, given n = 3, a solution set is:

// [
//   "((()))",
//   "(()())",
//   "(())()",
//   "()(())",
//   "()()()"
// ]

pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n <= 0 {
            return vec![];
        }
        let mut ans: Vec<String> = vec![];
        let mut cur_str = "".to_owned();
        Solution::dfs(&mut ans, &mut cur_str, 0, 0, n);
        ans
    }
    fn dfs(ans: &mut Vec<String>, cur_str: &mut String, left: i32, right: i32, n: i32) {
        if left == n && right == n {
            ans.push(cur_str.clone());
            return;
        }

        // 只能添加(
        if right >= left {
            cur_str.push('(');
            Solution::dfs(ans, cur_str, left + 1, right, n);
        } else {
            // 或许添加(
            if left + 1 <= n {
                cur_str.push('(');
                Solution::dfs(ans, &mut cur_str.clone(), left + 1, right, n);
                cur_str.pop();
            }
            // 或许添加)
            if right + 1 <= n {
                cur_str.push(')');
                Solution::dfs(ans, &mut cur_str.clone(), left, right + 1, n);
                cur_str.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_parenthesis_test() {
        let empty: Vec<String> = vec![];
        assert_eq!(Solution::generate_parenthesis(0), empty);
        assert_eq!(Solution::generate_parenthesis(-1), empty);

        assert_eq!(
            Solution::generate_parenthesis(2),
            vec!["(())".to_owned(), "()()".to_owned()]
        );

        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))".to_owned(),
                "(()())".to_owned(),
                "(())()".to_owned(),
                "()(())".to_owned(),
                "()()()".to_owned()
            ]
        );

        assert_eq!(
            Solution::generate_parenthesis(4),
            vec![
                "(((())))".to_owned(),
                "((()()))".to_owned(),
                "((())())".to_owned(),
                "((()))()".to_owned(),
                "(()(()))".to_owned(),
                "(()()())".to_owned(),
                "(()())()".to_owned(),
                "(())(())".to_owned(),
                "(())()()".to_owned(),
                "()((()))".to_owned(),
                "()(()())".to_owned(),
                "()(())()".to_owned(),
                "()()(())".to_owned(),
                "()()()()".to_owned()
            ]
        );
    }
}
