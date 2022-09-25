from typing import Optional

from helper import ListNode, create_link, trave_link


class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(-1)
        while head:
            cur = head
            head = head.next
            cur.next = dummy.next
            dummy.next = cur
        return dummy.next


if __name__ == "__main__":
    head1 = create_link([1, 2, 4])
    head = Solution().reverseList(head1)
    assert trave_link(head) == [4, 2, 1]

    head1 = create_link([1])
    head = Solution().reverseList(head1)
    assert trave_link(head) == [1]

    head1 = create_link([])
    head = Solution().reverseList(head1)
    assert trave_link(head) == []
