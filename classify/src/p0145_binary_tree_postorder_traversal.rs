// 145. Binary Tree Postorder Traversal
// Hard
// Given a binary tree, return the postorder traversal of its nodes' values.

// Example:

// Input: [1,null,2,3]
//    1
//     \
//      2
//     /
//    3

// Output: [3,2,1]
// Follow up: Recursive solution is trivial, could you do it iteratively?

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
pub const NULL: i32 = std::i32::MIN;

pub fn build_tree_from_vec(v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_by_recursive(0, v)
}
fn build_tree_by_recursive(i: usize, v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if i >= v.len() || v[i] == NULL {
        return None;
    }
    let left = build_tree_by_recursive(2 * i + 1, v);
    let right = build_tree_by_recursive(2 * i + 2, v);
    Some(Rc::new(RefCell::new(TreeNode {
        val: v[i],
        left,
        right,
    })))
}

// 把省略的NULL给填充
pub fn full_vec_with_null(v: &Vec<i32>) -> Vec<i32> {
    // 找到所有需要填充的index
    let tags = v.iter()
        .enumerate()
        .filter(|(_, &num)| num == NULL)
        .map(|(index, _)| index)
        .filter(|&index| index*2+2 <= v.len())
        .collect::<Vec<usize>>();
    let mut result = v.clone();
    for &tag in tags.iter() {
        result.insert(tag*2+1, NULL);
        result.insert(tag*2+2, NULL);
    }
    result
}

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Solution::internal(root, &mut v);
        return v;
    }
    fn internal(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(r) = root {
            Solution::internal(r.borrow().left.clone(), v);
            Solution::internal(r.borrow().right.clone(), v);
            v.push(r.borrow().val);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postorder_traversal_test() {
        assert_eq!(full_vec_with_null(&vec![1, NULL, NULL]), vec![1, NULL, NULL]);
        assert_eq!(full_vec_with_null(&vec![1, 2, NULL, 3]), vec![1, 2, NULL, 3]);
        assert_eq!(full_vec_with_null(&vec![1, 2, NULL, 3, 4]), vec![1, 2, NULL, 3, 4]);
        assert_eq!(full_vec_with_null(&vec![1, NULL, 2, 3]), vec![1, NULL, 2, NULL, NULL, 3]);
        assert_eq!(full_vec_with_null(&vec![1, NULL, 2, 3, NULL, NULL]), vec![1, NULL, 2, NULL, NULL, 3, NULL, NULL]);
        assert_eq!(full_vec_with_null(&vec![1, NULL, 2, 3, NULL]), vec![1, NULL, 2, NULL, NULL, 3, NULL]);

        let root = build_tree_from_vec(&full_vec_with_null(&vec![1, NULL, 2, 3]));
        assert_eq!(Solution::postorder_traversal(root), vec![3, 2, 1]);
    }
}
