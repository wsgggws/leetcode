# 剑指 Offer 06. 从尾到头打印链表
# 输入一个链表的头节点，从尾到头反过来返回每个节点的值（用数组返回）。


# 示例 1：

# 输入：head = [1,3,2]
# 输出：[2,3,1]


# 限制：

# 0 <= 链表长度 <= 10000

from typing import List


# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution:
    def reversePrint(self, head: ListNode) -> List[int]:
        """递归实现与非递归实现
        递归调用时后面的变量语句会先压栈,    递归结束时再弹出栈
        []                                  []
        [2]                                 [2]         => [] + [2] + [3] + [1] => [2, 3, 1]
        [3]                                 [3]
        [1]                                 [1]
        """
        # if head is None:
        #     return []
        # return self.reversePrint(head.next) + [head.val]

        stack = []

        while head is not None:
            stack.append(head.val)
            head = head.next

        return stack[::-1]


if __name__ == "__main__":
    head = ListNode(1)
    head.next = ListNode(3)
    head.next.next = ListNode(2)
    s = Solution()
    assert s.reversePrint(head) == [2, 3, 1]
