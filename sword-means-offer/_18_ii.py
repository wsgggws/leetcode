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
    head = ListNode(nums[0])
    cur = head
    for num in nums[1:]:
        node = ListNode(num)
        cur.next = node
        cur = node
    return head


def trave_link(head: ListNode) -> List[int]:
    nums = []
    if head is None:
        return nums
    # [h] -> [] -> None
    while head:
        nums.append(head.val)
        head = head.next
    return nums


class Solution:
    # def deleteNode(self, head: ListNode, val: int) -> ListNode:
    #     """
    #     非递归实现
    #     """
    #     if head.val == val:
    #         return head.next
    #     # [pre] -> [cur] -> []
    #     cur = head.next
    #     pre = head
    #     while cur:
    #         if cur.val == val:
    #             pre.next = pre.next.next
    #             break
    #         pre = cur
    #         cur = cur.next
    #     return head

    def deleteNode(self, head: ListNode, val: int) -> ListNode:
        """
        非递归实现, 哑结点
        """
        #   cur
        # [dumb] -> [cur.next] -> [] -> [] -> None
        dumb_head = ListNode(-1)
        dumb_head.next = head
        cur = dumb_head
        while cur and cur.next:
            if cur.next.val == val:
                cur.next = cur.next.next
                break
            cur = cur.next
        return dumb_head.next


if __name__ == "__main__":
    nums = [4, 5, 1, 9]
    head = create_link(nums)
    assert trave_link(head) == [4, 5, 1, 9]
    assert trave_link(head) == [4, 5, 1, 9]
    s = Solution()
    head = s.deleteNode(head, 5)
    assert trave_link(head) == [4, 1, 9]
    head = s.deleteNode(head, 0)
    assert trave_link(head) == [4, 1, 9]
    head = s.deleteNode(head, 4)
    assert trave_link(head) == [1, 9]
    head = s.deleteNode(head, 9)
    assert trave_link(head) == [1]
    head = s.deleteNode(head, 1)
    assert head is None
    assert trave_link(head) == []
