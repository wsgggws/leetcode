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
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        fast, slow, pre = head, head, head
        while n > 0:
            fast = fast.next
            n -= 1
        while fast:
            fast = fast.next
            pre = slow
            slow = slow.next
        if pre == slow:
            head = slow.next
            slow.next = None
        pre.next = slow.next
        slow.next = None
        return head


if __name__ == "__main__":
    head = create_link([1, 2, 3, 4, 5])
    assert trave_link(head) == [1, 2, 3, 4, 5]
    head = Solution().removeNthFromEnd(head, 2)
    assert trave_link(head) == [1, 2, 3, 5]

    head = create_link([1])
    head = Solution().removeNthFromEnd(head, 1)
    assert trave_link(head) == []

    head = create_link([1, 2])
    head = Solution().removeNthFromEnd(head, 1)
    assert trave_link(head) == [1]

    head = create_link([1, 2])
    head = Solution().removeNthFromEnd(head, 2)
    assert trave_link(head) == [2]
