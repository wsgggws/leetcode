from typing import List, Optional
from helper import ListNode, create_link, trave_link


class Solution:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        """归并排序
        1. 从中间分割(快慢指针)
        2. 合并链表
        """
        return None if head is None else self.mergeSort(head)

    def mergeSort(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head and head.next is None:
            return head
        fast, slow, pre = head, head, None
        while fast and fast.next and slow and slow.next:
            pre = slow
            fast = fast.next.next
            slow = slow.next
        pre.next = None
        left = self.mergeSort(head)
        right = self.mergeSort(slow)
        return self.merge(left, right)

    def merge(self, left: ListNode, right: ListNode) -> ListNode:
        dump = ListNode(-1)
        cur = dump
        while left and right:
            if left.val <= right.val:
                cur.next = left
                left = left.next
            else:
                cur.next = right
                right = right.next
            cur = cur.next
        if left:
            cur.next = left
        if right:
            cur.next = right
        return dump.next


if __name__ == "__main__":
    head = create_link([4, 2, 1, 3])
    assert trave_link(Solution().sortList(head)) == [1, 2, 3, 4]
    head = create_link([-1, 5, 3, 4, 0])
    assert trave_link(Solution().sortList(head)) == [-1, 0, 3, 4, 5]
    head = create_link([])
    assert trave_link(Solution().sortList(head)) == []
