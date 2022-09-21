from typing import List


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


def connect_link(head_a: ListNode, head_b: ListNode) -> ListNode:
    cur = head_a
    if cur is None:
        return head_b
    while cur and cur.next is not None:
        cur = cur.next
    cur.next = head_b
    return head_a


def trave_link(head: ListNode) -> List[int]:
    if head is None:
        return []
    if head.next is None:
        return [head.val]
    return [head.val] + trave_link(head.next)


class Solution:
    # def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> ListNode:
    #     """
    #     stackA, pop
    #     stackB, pop
    #     """
    #     stack_a = []
    #     cur = headA
    #     while cur is not None:
    #         stack_a.append(cur)
    #         cur = cur.next

    #     stack_b = []
    #     cur = headB
    #     while cur is not None:
    #         stack_b.append(cur)
    #         cur = cur.next

    #     ans = None
    #     while True:
    #         if len(stack_a) == 0 or len(stack_b) == 0:
    #             return ans
    #         node_a = stack_a.pop()
    #         node_b = stack_b.pop()
    #         if node_a is node_b:
    #             ans = node_a
    #             continue
    #         else:
    #             return ans
    #     return ans

    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> ListNode:
        """
        A(a+c) + B(b+c) + null
        B(b+c) + A(a+c) + null
        """
        cur_a, cur_b = headA, headB
        while cur_a is not cur_b:
            if cur_a is None:
                cur_a = headB
            else:
                cur_a = cur_a.next

            if cur_b is None:
                cur_b = headA
            else:
                cur_b = cur_b.next
        return cur_a


if __name__ == "__main__":

    common = create_link([8, 4, 5])
    head = connect_link(None, common)
    assert head is common
    assert trave_link(head) == trave_link(common)

    head = connect_link(common, None)
    assert head is common
    assert trave_link(head) == trave_link(common)

    s = Solution()
    head_a = create_link([4, 1])
    head_b = create_link([5, 0, 1])
    head_ans = s.getIntersectionNode(head_a, head_b)
    assert head_ans is None
    assert trave_link(head_ans) == []

    head_a = connect_link(head_a, common)
    head_b = connect_link(head_b, common)
    assert trave_link(head_a) == [4, 1, 8, 4, 5]
    assert trave_link(head_b) == [5, 0, 1, 8, 4, 5]

    head_ans = s.getIntersectionNode(head_a, head_b)
    assert head_ans is common
    assert trave_link(head_ans) == trave_link(common)

    head_ans = s.getIntersectionNode(common, common)
    assert head_ans is common
    assert trave_link(head_ans) == trave_link(common)

    head_ans = s.getIntersectionNode(None, None)
    assert head_ans is None
    assert trave_link(head_ans) == []

    common = create_link([1])
    head_a = connect_link(None, common)
    head_b = connect_link(None, common)
    head_ans = s.getIntersectionNode(head_a, head_b)
    assert head_ans is common
    assert trave_link(head_ans) == [1]
