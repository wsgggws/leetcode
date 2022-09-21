// 155. Min Stack
// Easy

// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

// push(x) -- Push element x onto stack.
// pop() -- Removes the element on top of the stack.
// top() -- Get the top element.
// getMin() -- Retrieve the minimum element in the stack.

// Example 1:

// Input
// ["MinStack","push","push","push","getMin","pop","top","getMin"]
// [[],[-2],[0],[-3],[],[],[],[]]

// Output
// [null,null,null,null,-3,null,0,-2]

// Explanation
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin(); // return -3
// minStack.pop();
// minStack.top();    // return 0
// minStack.getMin(); // return -2

// Constraints:

// Methods pop, top and getMin operations will alays be called on non-empty stacks.w

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

pub struct MinStack {
    stack: Vec<i32>,
}

impl MinStack {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        MinStack { stack: vec![] }
    }

    pub fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        let lens = self.stack.len();
        if lens > 0 {
            self.stack[lens - 1]
        } else {
            -1
        }
    }
    pub fn get_min(&self) -> i32 {
        *self.stack.iter().min().unwrap_or(&-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_stack_test() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
