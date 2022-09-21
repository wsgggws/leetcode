// 144. Binary Tree Preorder Traversal
// Medium

// Given a binary tree, return the preorder traversal of its nodes' values.

// Example:

// Input: [1,null,2,3]
//    1
//     \
//      2
//     /
//    3

// Output: [1,2,3]
// Follow up: Recursive solution is trivial, could you do it iteratively?

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
use crate::p0145_binary_tree_postorder_traversal::TreeNode;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        Solution::pre_order(root, &mut result);
        result
    }

    pub fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            result.push(node.borrow().val);
            Solution::pre_order(node.borrow().left.clone(), result);
            Solution::pre_order(node.borrow().right.clone(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::p0145_binary_tree_postorder_traversal::build_tree_from_vec;
    use crate::p0145_binary_tree_postorder_traversal::NULL;
    #[test]
    fn preorder_traversal_test() {
        let root = build_tree_from_vec(&vec![1, NULL, 2, NULL, NULL, 3]);
        assert_eq!(Solution::preorder_traversal(root), vec![1, 2, 3]);
    }
}
