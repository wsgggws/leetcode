from typing import Optional

from helper import ListNode, create_link, trave_link


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
