// 150. Evaluate Reverse Polish Notation
// Medium

// Evaluate the value of an arithmetic expression in Reverse Polish Notation.

// Valid operators are +, -, *, /. Each operand may be an integer or another expression.

// Note:

// Division between two integers should truncate toward zero.
// The given RPN expression is always valid. That means the expression would always evaluate to a result and there won't be any divide by zero operation.
// Example 1:

// Input: ["2", "1", "+", "3", "*"]
// Output: 9
// Explanation: ((2 + 1) * 3) = 9
// Example 2:

// Input: ["4", "13", "5", "/", "+"]
// Output: 6
// Explanation: (4 + (13 / 5)) = 6
// Example 3:

// Input: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
// Output: 22
// Explanation:
//   ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
// = ((10 * (6 / (12 * -11))) + 17) + 5
// = ((10 * (6 / -132)) + 17) + 5
// = ((10 * 0) + 17) + 5
// = (0 + 17) + 5
// = 17 + 5
// = 22

pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for a_str in tokens.into_iter() {
            // TODO 如何使用match实现
            // match a_str.as_ref() {
            //     op @ "+" | "-" | "*" | "/" => {
            //         let a = stack.pop().unwrap();
            //         let b = stack.pop().unwrap();
            //         match op {
            //             "+" => { stack.push(a+b); },
            //             "-" => { stack.push(a-b); },
            //             "*" => { stack.push(a*b); },
            //             "/" => { stack.push(a/b); },
            //             _ => {},
            //         }
            //     },
            //     op @ _ => {
            //         stack.push(a_str.clone().parse::<i32>().unwrap());
            //     }
            // }
            if a_str == "+".to_owned() {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            } else if a_str == "-".to_owned() {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            } else if a_str == "*".to_owned() {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            } else if a_str == "/".to_owned() {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b / a);
            } else {
                stack.push(a_str.parse::<i32>().unwrap());
            }
        }
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_rpn_test() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".to_owned(),
                "1".to_owned(),
                "+".to_owned(),
                "3".to_owned(),
                "*".to_owned(),
            ]),
            9
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".to_owned(),
                "13".to_owned(),
                "5".to_owned(),
                "/".to_owned(),
                "+".to_owned(),
            ]),
            6
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_owned(),
                "6".to_owned(),
                "9".to_owned(),
                "3".to_owned(),
                "+".to_owned(),
                "-11".to_owned(),
                "*".to_owned(),
                "/".to_owned(),
                "*".to_owned(),
                "17".to_owned(),
                "+".to_owned(),
                "5".to_owned(),
                "+".to_owned(),
            ]),
            22
        );
    }
}
