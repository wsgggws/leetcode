# 剑指 Offer 24. 反转链表
# 定义一个函数，输入一个链表的头节点，反转该链表并输出反转后链表的头节点。


# 示例:

# 输入: 1->2->3->4->5->NULL
# 输出: 5->4->3->2->1->NULL


# 限制：

# 0 <= 节点个数 <= 5000


from typing import List


# Definition for singly-linked list.
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
    # def reverseList(self, head: ListNode) -> ListNode:
    #     """递归
    #     stack
    #     [4].next.next = [4], [1].next = None  => 4 先返回执行
    #     [3].next.next = [3], [1].next = None
    #     [2].next.next = [2], [1].next = None
    #     [1].next.next = [1], [1].next = None
    #     """
    #     if head is None or head.next is None:
    #         return head
    #     node = self.reverseList(head.next)
    #     # [head] -> [head.next] -> [...] -> None
    #     head.next.next = head
    #     head.next = None
    #     return node
    def reverseList(self, head: ListNode) -> ListNode:
        """迭代"""
        # pre=None, [cur] -> [nex] -> None
        # [pre] -> [cur] -> [nex] -> None
        pre = None
        cur = head
        while cur is not None:
            nex = cur.next
            cur.next = pre
            pre = cur
            cur = nex
        return pre


if __name__ == "__main__":
    nums = [1, 2, 3, 4, 5]
    head = create_link(nums)
    assert trave_link(head) == [1, 2, 3, 4, 5]
    s = Solution()
    head = s.reverseList(head)
    assert trave_link(head) == [5, 4, 3, 2, 1]
    nums = []
    head = create_link(nums)
    assert trave_link(head) == []
