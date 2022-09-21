from typing import List, Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


def create_link(nums: List[int]) -> Optional[ListNode]:
    if len(nums) == 0:
        return None
    head = ListNode(nums[0])
    cur = head
    for num in nums[1:]:
        node = ListNode(num)
        cur.next = node
        cur = node
    return head


def trave_link(head: Optional[ListNode]) -> List[int]:
    nums = []
    if head is None:
        return nums
    # [h] -> [] -> None
    while head:
        nums.append(head.val)
        head = head.next
    return nums


class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        fast, slow = head, head
        while fast:
            fast = fast.next
            if fast:
                fast = fast.next
                slow = slow.next
        dump = ListNode(-1)
        cur = slow
        while cur:
            next_node = cur.next
            cur.next = dump.next
            dump.next = cur
            cur = next_node
        head2 = dump.next
        head1 = head
        while head1 and head2:
            if head1.val != head2.val:
                return False
            head1 = head1.next
            head2 = head2.next
        return True


if __name__ == "__main__":
    head = create_link([1, 2, 3, 4, 5])
    assert trave_link(head) == [1, 2, 3, 4, 5]
    assert Solution().isPalindrome(head) is False

    head = create_link([1, 2, 2, 1])
    assert Solution().isPalindrome(head) is True

    head = create_link([1, 2, 3, 2, 1])
    assert Solution().isPalindrome(head) is True

    head = create_link([1, 2])
    assert Solution().isPalindrome(head) is False

    head = create_link([1])
    assert Solution().isPalindrome(head) is True
