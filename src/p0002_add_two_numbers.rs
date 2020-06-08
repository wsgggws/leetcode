// 2. Add Two Numbers
// Medium

// 8049

// 2052

// Add to List

// Share
// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Example:

// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
// Output: 7 -> 0 -> 8
// Explanation: 342 + 465 = 807.

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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut a, mut b, mut carry) = (l1, l2, 0);
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();

        while a.is_some() || b.is_some() {

            let mut sum = carry;

            if let Some(v) = a {
                sum += (*v).val;
                a = v.next;
            }

            if let Some(v) = b {
                sum += (*v).val;
                b = v.next;
            }
            carry = sum / 10;
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(sum % 10)));
            current = v.next.as_mut();
        }
        if carry > 0 {
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(carry)));
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
