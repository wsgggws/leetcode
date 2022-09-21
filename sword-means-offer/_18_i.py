# 剑指 Offer 18. 删除链表的节点
# 给定单向链表的头指针和一个要删除的节点的值，定义一个函数删除该节点。

# 返回删除后的链表的头节点。

# 注意：此题对比原题有改动

# 示例 1:

# 输入: head = [4,5,1,9], val = 5
# 输出: [4,1,9]
# 解释: 给定你链表中值为 5 的第二个节点，那么在调用了你的函数之后，该链表应变为 4 -> 1 -> 9.
# 示例 2:

# 输入: head = [4,5,1,9], val = 1
# 输出: [4,5,9]
# 解释: 给定你链表中值为 1 的第三个节点，那么在调用了你的函数之后，该链表应变为 4 -> 5 -> 9.

# 说明：

# 题目保证链表中节点的值互不相同
# 若使用 C 或 C++ 语言，你不需要 free 或 delete 被删除的节点


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
    def deleteNode(self, head: ListNode, val: int) -> ListNode:
        if head is None:
            return None
        if head.val == val:
            return head.next
        head.next = self.deleteNode(head.next, val)
        return head


if __name__ == "__main__":
    nums = [4, 5, 1, 9]
    head = create_link(nums)
    assert trave_link(head) == [4, 5, 1, 9]
    s = Solution()
    head = s.deleteNode(head, 5)
    assert trave_link(head) == [4, 1, 9]
    head = s.deleteNode(head, 4)
    assert trave_link(head) == [1, 9]
    head = s.deleteNode(head, 9)
    assert trave_link(head) == [1]
    head = s.deleteNode(head, 1)
    assert head is None
    assert trave_link(head) == []
    nums = []
    head = create_link(nums)
    assert trave_link(head) == []
