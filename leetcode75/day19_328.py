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
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None
        first, second, head2 = head, head.next, head.next
        while first and first.next and second and second.next:
            first.next = first.next.next
            second.next = second.next.next
            first = first.next
            second = second.next
        first.next = head2
        return head


if __name__ == "__main__":
    head = create_link([1, 2, 3, 4, 5])
    assert trave_link(head) == [1, 2, 3, 4, 5]
    head = Solution().oddEvenList(head)
    assert trave_link(head) == [1, 3, 5, 2, 4]

    head = create_link([1, 2, 3, 4, 5, 6])
    head = Solution().oddEvenList(head)
    assert trave_link(head) == [1, 3, 5, 2, 4, 6]

    head = create_link([])
    head = Solution().oddEvenList(head)
    assert trave_link(head) == []

    head = create_link([1])
    head = Solution().oddEvenList(head)
    assert trave_link(head) == [1]

    head = create_link([1, 2])
    head = Solution().oddEvenList(head)
    assert trave_link(head) == [1, 2]

    head = create_link([1, 2, 3])
    head = Solution().oddEvenList(head)
    assert trave_link(head) == [1, 3, 2]
