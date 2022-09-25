from typing import Optional

from helper import ListNode, create_link, trave_link


class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        slow, fast = head, head
        while fast:
            fast = fast.next
            if not fast:
                return slow
            fast = fast.next
            slow = slow.next
        return slow


if __name__ == "__main__":
    head1 = create_link([1, 2, 3, 4, 5])
    head = Solution().middleNode(head1)
    assert trave_link(head) == [3, 4, 5]

    head1 = create_link([1, 2, 3, 4, 5, 6])
    head = Solution().middleNode(head1)
    assert trave_link(head) == [4, 5, 6]

    head1 = create_link([1])
    head = Solution().middleNode(head1)
    assert trave_link(head) == [1]

    head1 = create_link([1, 2])
    head = Solution().middleNode(head1)
    assert trave_link(head) == [2]
