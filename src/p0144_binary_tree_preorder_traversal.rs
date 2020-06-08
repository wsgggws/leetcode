// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        Solution::pre_order(root, &mut result);
        result
    }

    pub fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            result.push((*node).borrow().val);
            Solution::pre_order((*node).borrow().left.clone(), result);
            Solution::pre_order((*node).borrow().right.clone(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn preorder_traversal_test() {
        // TODO TEST
        assert_eq!(2 + 2, 4)
    }
}
