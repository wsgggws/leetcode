from typing import Optional

from helper import ListNode, create_link, trave_link


class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        fast, slow, pre = head, head, head
        while n > 0 and fast:
            fast = fast.next
            n -= 1
        while fast and slow:
            fast = fast.next
            pre = slow
            slow = slow.next
        if pre and slow and pre == slow:
            head = slow.next
            slow.next = None
        if pre and slow:
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
