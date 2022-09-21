from typing import List


class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


def create_link(nums: List[int]) -> ListNode:
    if len(nums) == 0:
        return None
    if len(nums) == 1:
        return ListNode(nums[0])
    node = ListNode(nums[0])
    node.next = create_link(nums[1:])
    return node


def trave_link(head: ListNode) -> List[int]:
    if head is None:
        return []
    if head.next is None:
        return [head.val]
    return [head.val] + trave_link(head.next)


class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        """
        1,2,4
        1,3,4
        """
        head = ListNode(0)
        cur = head
        cur1, cur2 = l1, l2
        while cur1 and cur2:
            if cur1.val <= cur2.val:
                cur.next = cur1
                cur1 = cur1.next
            else:
                cur.next = cur2
                cur2 = cur2.next
            cur = cur.next
        if cur1 is not None:
            cur.next = cur1
        if cur2 is not None:
            cur.next = cur2

        return head.next


if __name__ == "__main__":
    s = Solution()
    l1 = create_link([1, 2, 4])
    l2 = create_link([1, 3, 4])
    ans = s.mergeTwoLists(l1, l2)
    assert trave_link(ans) == [1, 1, 2, 3, 4, 4]

    l1 = create_link([])
    l2 = create_link([1, 3, 4])
    ans = s.mergeTwoLists(l1, l2)
    assert trave_link(ans) == [1, 3, 4]

    l1 = create_link([1, 3, 4])
    l2 = create_link([])
    ans = s.mergeTwoLists(l1, l2)
    assert trave_link(ans) == [1, 3, 4]

    l1 = create_link([])
    l2 = create_link([])
    ans = s.mergeTwoLists(l1, l2)
    assert trave_link(ans) == []
