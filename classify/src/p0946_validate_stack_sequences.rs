// 946. Validate Stack Sequences
// Medium

// Given two sequences pushed and popped with distinct values, return true if and only if this could have been the result of a sequence of push and pop operations on an initially empty stack.

 

// Example 1:

// Input: pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
// Output: true
// Explanation: We might do the following sequence:
// push(1), push(2), push(3), push(4), pop() -> 4,
// push(5), pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
// Example 2:

// Input: pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
// Output: false
// Explanation: 1 cannot be popped before 2.
 

// Note:

// 0 <= pushed.length == popped.length <= 1000
// 0 <= pushed[i], popped[i] < 1000
// pushed is a permutation of popped.
// pushed and popped have distinct values.

pub struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack_pushed: Vec<i32> = vec![];
        let mut index_popped = 0usize;
        let mut index_pushed = 0usize;
        while index_pushed < pushed.len() && index_popped < popped.len() {
            if pushed[index_pushed] == popped[index_popped] {
                index_popped += 1;
                index_pushed += 1;
            } else {
                if stack_pushed.len() > 0 && stack_pushed[stack_pushed.len()-1] == popped[index_popped] {
                    stack_pushed.pop();
                    index_popped += 1;
                } else {
                    stack_pushed.push(pushed[index_pushed]);
                    index_pushed += 1;
                }
            }
        }
        while let Some(num) = stack_pushed.pop() {
            if num == popped[index_popped] {
                index_popped += 1;
            } else {
                return false;
            }
        }
        // println!("index_popped: {}, stack_pushed: {:?}", index_popped, stack_pushed);
        index_popped == popped.len() && stack_pushed.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_stack_sequences_test() {
        assert_eq!(Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]), true);
        assert_eq!(Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]), false);
        assert_eq!(Solution::validate_stack_sequences(vec![2, 1, 0], vec![1, 2, 0]), true);
    }
}
