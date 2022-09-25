from typing import Optional

from helper import ListNode, create_ring_link, trave_ring_link


class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        cycle = False
        slow, fast = head, head
        while fast and fast.next and slow:
            fast = fast.next.next
            slow = slow.next
            if fast == slow:
                cycle = True
                break
        if cycle is False:
            return None
        # 有环
        result = head
        while result and slow and result != slow:
            result = result.next
            slow = slow.next
        return result


if __name__ == "__main__":
    head1 = create_ring_link([3, 2, 0, 4], 1)
    head = Solution().detectCycle(head1)
    assert trave_ring_link(head)[:6] == [2, 0, 4, 2, 0, 4]

    head1 = create_ring_link([1, 2], 0)
    head = Solution().detectCycle(head1)
    assert trave_ring_link(head)[:4] == [1, 2, 1, 2]

    head1 = create_ring_link([1], -1)
    head = Solution().detectCycle(head1)
    assert trave_ring_link(head) == []
