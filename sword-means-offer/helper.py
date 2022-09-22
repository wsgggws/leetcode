from typing import Optional, List

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


def create_link(nums: List[int]) -> Optional[ListNode]:
    if len(nums) == 0:
        return None
    head = ListNode(nums[0])
    cur = head
    for num in nums[1:]:
        node = ListNode(num)
        cur.next = node
        cur = node
    return head


def trave_link(head: Optional[ListNode]) -> List[int]:
    nums = []
    if head is None:
        return nums
    # [h] -> [] -> None
    while head:
        nums.append(head.val)
        head = head.next
    return nums


def create_ring_link(nums: List[int], pos) -> Optional[ListNode]:
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


def trave_ring_link(head: Optional[ListNode]) -> List[int]:
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


class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left: Optional[TreeNode] = None
        self.right: Optional[TreeNode] = None


def create_binary_tree(nums: List[int], index: int) -> Optional[TreeNode]:
    """
         1
      2     2
    3   4 4   3
    左右结点分别为 2*i+1, 2*i+2
    """
    # 递归创建二叉树
    if index >= len(nums) or nums[index] is None:
        return
    root = TreeNode(nums[index])
    root.left = create_binary_tree(nums, index * 2 + 1)
    root.right = create_binary_tree(nums, index * 2 + 2)
    return root


def bfs_binary_tree(root: Optional[TreeNode]) -> List[int]:
    # 层序遍历
    if root is None:
        return []
    queue = [root]
    results = []
    while queue:
        size = len(queue)
        nums = []
        next_queue = []
        for index in range(size):
            node = queue[index]
            nums.append(node.val if node else None)
            if node:
                next_queue.append(node.left)
                next_queue.append(node.right)
        if any(num is not None for num in nums):
            results.extend(nums)
        queue = next_queue
    return results
