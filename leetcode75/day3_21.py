from typing import List, Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val: int = val
        self.next: Optional[ListNode] = next


def create_link(nums: List[int]) -> Optional[ListNode]:
    """创建链表"""
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
    """遍历链表"""
    nums = []
    if head is None:
        return nums
    while head:
        nums.append(head.val)
        head = head.next
    return nums


class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        head = ListNode(-1)
        h1, h2 = list1, list2
        cur = head
        while h1 and h2:
            if h1.val <= h2.val:
                cur.next = h1
                h1 = h1.next
            else:
                cur.next = h2
                h2 = h2.next
            cur = cur.next
        if h1:
            cur.next = h1
        else:
            cur.next = h2
        return head.next


if __name__ == "__main__":
    head1 = create_link([1, 2, 4])
    head2 = create_link([1, 3, 4])
    head = Solution().mergeTwoLists(head1, head2)
    assert trave_link(head) == [1, 1, 2, 3, 4, 4]

    head1 = create_link([])
    head2 = create_link([])
    head = Solution().mergeTwoLists(head1, head2)
    assert trave_link(head) == []

    head1 = create_link([1, 2, 4])
    head2 = create_link([])
    head = Solution().mergeTwoLists(head1, head2)
    assert trave_link(head) == [1, 2, 4]

    head1 = create_link([1, 2, 4])
    head2 = create_link([3])
    head = Solution().mergeTwoLists(head1, head2)
    assert trave_link(head) == [1, 2, 3, 4]
