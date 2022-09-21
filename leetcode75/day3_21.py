from typing import Optional
from helper import ListNode, create_link, trave_link


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
