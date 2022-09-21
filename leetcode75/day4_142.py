from typing import List, Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val: int = val
        self.next: Optional[ListNode] = next


def create_link(nums: List[int], pos) -> Optional[ListNode]:
    """创建链表"""
    if len(nums) == 0:
        return None
    head = ListNode(nums[0])
    cur = head
    for num in nums[1:]:
        node = ListNode(num)
        cur.next = node
        cur = node

    # 无环
    if pos == -1:
        return head
    # 有环，找到交点，尾结点指向它
    node = head
    while pos > 0 and node:
        node = node.next
        pos -= 1
    cur.next = node
    return head


def trave_link(head: Optional[ListNode]) -> List[int]:
    """遍历链表"""
    nums = []
    if head is None:
        return nums
    count = 10
    while head and count:
        nums.append(head.val)
        head = head.next
        count -= 1
    return nums


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
    head1 = create_link([3, 2, 0, 4], 1)
    head = Solution().detectCycle(head1)
    assert trave_link(head)[:6] == [2, 0, 4, 2, 0, 4]

    head1 = create_link([1, 2], 0)
    head = Solution().detectCycle(head1)
    assert trave_link(head)[:4] == [1, 2, 1, 2]

    head1 = create_link([1], -1)
    head = Solution().detectCycle(head1)
    assert trave_link(head) == []
