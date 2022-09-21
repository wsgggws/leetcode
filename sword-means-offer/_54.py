class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


def create_binary_tree(nums: list, index: int) -> TreeNode:
    if index >= len(nums) or nums[index] is None:
        return
    node = TreeNode(nums[index])
    node.left = create_binary_tree(nums, index * 2 + 1)
    node.right = create_binary_tree(nums, index * 2 + 2)
    return node


class Solution:
    def kthLargest(self, root: TreeNode, k: int) -> int:
        """二叉搜索树： 任意一个结点，左子结点的值小于根结点，左子结点的值大于根结点
        搜索：是相对根结点而言的，然后左为大(在中国也无时无不体现着左为大)
        前序遍历(PreOder): 根左右  （3 1 2 4）
        中序遍历(MidOrder): 左根右（1 2 3 4）
        后序遍布(PostOrder): 左右根 （2 1 4 3）
        迭代实现
                 5
                / \
               3   6
              / \
             2   4
            /
           1


            [1] [1,2]
                  [1,2,3]
                     [1,2,3,4]
                       [1,2,3,4,5]
                            [1,2,3,4,5,6]
        1
        2   2
        3   3    3   4
        5   5    5   5  5  _  6  _
        """
        stack = []
        nums = []
        node = root
        while node or stack:
            # 处理左结点压入栈的过程
            while node:
                stack.append(node)
                node = node.right
            # 处理根结点，及 right 结点
            node = stack.pop()
            nums.append(node.val)
            if len(nums) >= k:
                return nums[k - 1]
            node = node.left
        return nums[k - 1]

    # def kthLargest(self, root: TreeNode, k: int) -> int:
    #     """二叉搜索树： 任意一个结点，左子结点的值小于根结点，左子结点的值大于根结点
    #     搜索：是相对根结点而言的，然后左为大(在中国也无时无不体现着左为大)
    #     前序遍历(PreOder): 根左右  （3 1 2 4）
    #     变化的中序遍历(MidOrder): 右根左 （4 3 2 1）
    #     后序遍布(PostOrder): 左右根 （2 1 4 3）
    #     """
    #     nums = []
    #     self._changed_mid_order(root, k, nums)

    #     return nums[k - 1]

    # def _changed_mid_order(self, root: TreeNode, k: int, nums: List[int]) -> None:
    #     if root is None:
    #         return
    #     if len(nums) >= k:
    #         return
    #     self._changed_mid_order(root.right, k, nums)
    #     nums.append(root.val)
    #     self._changed_mid_order(root.left, k, nums)

    # def kthLargest(self, root: TreeNode, k: int) -> int:
    #     """二叉搜索树： 任意一个结点，左子结点的值小于根结点，左子结点的值大于根结点
    #     搜索：是相对根结点而言的，然后左为大(在中国也无时无不体现着左为大)
    #     前序遍历(PreOder): 根左右  （3 1 2 4）
    #     中序遍历(MidOrder): 左根右 （1 2 3 4）
    #     后序遍布(PostOrder): 左右根 （2 1 4 3）
    #     """
    #     nums = []
    #     self._mid_order(root, k, nums)
    #     return nums[-k]

    # def _mid_order(self, root: TreeNode, k: int, nums: List[int]) -> None:
    #     if root is None:
    #         return
    #     self._mid_order(root.left, k, nums)
    #     nums.append(root.val)
    #     self._mid_order(root.right, k, nums)


if __name__ == "__main__":
    s = Solution()
    root = create_binary_tree([3, 1, 4, None, 2], 0)
    assert s.kthLargest(root, 1) == 4

    root = create_binary_tree([5, 3, 6, 2, 4, None, None, 1], 0)
    assert s.kthLargest(root, 3) == 4

    root = create_binary_tree([1], 0)
    assert s.kthLargest(root, 1) == 1
