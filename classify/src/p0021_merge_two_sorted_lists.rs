// 21. Merge Two Sorted Lists
// Easy

// Merge two sorted linked lists and return it as a new sorted list. The new list should be made by splicing together the nodes of the first two lists.

// Example:

// Input: 1->2->4, 1->3->4
// Output: 1->1->2->3->4->4

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>,) -> Option<Box<ListNode>> {
        let (mut a, mut b) = (l1, l2);
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();

        while a.is_some() && b.is_some() {
            // let mut value = 0;
            let value_a = a.as_ref().unwrap().val;
            let value_b = b.as_ref().unwrap().val;
            if value_a < value_b {
                if let Some(v) = a {
                    a = v.next;
                }
            } else {
                if let Some(v) = b {
                    b = v.next;
                }
            }
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(i32::min(value_a, value_b))));
            current = v.next.as_mut();
        }
        while a.is_some() {
            let value_a = a.as_ref().unwrap().val;
            if let Some(v) = a {
                a = v.next;
            }
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(value_a)));
            current = v.next.as_mut();
        }
        while b.is_some() {
            let value_b = b.as_ref().unwrap().val;
            if let Some(v) = b {
                b = v.next;
            }
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(value_b)));
            current = v.next.as_mut();

        }
        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn add_two_numbers_test() {
        // TODO? don't know how to test it.
        assert_eq!(2 + 3, 5);
    }
}
