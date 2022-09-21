from typing import List


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


def bfs(root: TreeNode) -> list:
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


def preorder(root: TreeNode, nums) -> None:
    if root is None:
        return None
    nums.append(root.val)
    preorder(root.left, nums)
    preorder(root.right, nums)


def inorder(root: TreeNode, nums) -> None:
    if root is None:
        return None
    inorder(root.left, nums)
    nums.append(root.val)
    inorder(root.right, nums)


def postorder(root: TreeNode, nums) -> None:
    if root is None:
        return None
    postorder(root.left, nums)
    postorder(root.right, nums)
    nums.append(root.val)


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> TreeNode:
        """
        1. 前序遍历： 根左右，那能够确定根结点
        2. 中序遍历: 左根右， 那么根据前序遍历能区分左右子树
        对左右子树分别（递归） step1, 2, 即可还原出二叉树
        i.e.
        1. 根据 preorder[0] == 3, 确定 3 是根结点
        2. 根据根结点 3 划分中序遍历左右子树
            inorder[pos] 已经是 root 结点了
            inorder_left:  [9]
            inorder_right: [15, 20, 7]
        3. 根据 inorder_left 的长度划分前序遍历 preorder 的左右子树
            size(inorder_left) == 1
            3 已经是根结点，应当忽略
            preorder_left:  [9]
            preorder_right: [20, 15, 7]
        4. 递归构建左右子树
            root.left = buildTree(preorder_left, inorder_left)
            root.right = buildTree(preorder_right, inorder_right)
        """
        if len(preorder) == 0:
            return None
        if len(preorder) == 1:
            return TreeNode(preorder[0])

        root = TreeNode(preorder[0])
        # root.left, root.right
        pos = inorder.index(preorder[0])

        inorder_left = inorder[:pos]
        inorder_right = inorder[pos + 1 :]

        size = len(inorder_left)
        preorder_left = preorder[1 : size + 1]
        preorder_right = preorder[size + 1 :]

        root.left = self.buildTree(preorder_left, inorder_left)
        root.right = self.buildTree(preorder_right, inorder_right)

        return root


if __name__ == "__main__":
    s = Solution()
    root = s.buildTree([3, 9, 20, 15, 7], [9, 3, 15, 20, 7])

    nums = []
    preorder(root, nums)
    assert nums == [3, 9, 20, 15, 7]

    nums = []
    inorder(root, nums)
    assert nums == [9, 3, 15, 20, 7]

    nums = []
    postorder(root, nums)

    assert nums == [9, 15, 7, 20, 3]

    assert bfs(root) == [3, 9, 20, None, None, 15, 7]

    root = s.buildTree([-1], [-1])
    assert bfs(root) == [-1]

    root = s.buildTree([], [])
    assert bfs(root) == []
