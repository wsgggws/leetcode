from typing import List


class Solution:
    def verifyPostorder(self, postorder: List[int]) -> bool:
        if len(postorder) <= 1:
            return True
        root = postorder[-1]
        index = 0
        # 得出左，右子树
        while postorder[index] < root:
            index += 1

        left = postorder[:index]
        right = postorder[index:-1]

        # 左子树已经全小于 root, 现在检查右子树是否全大于根结点
        while postorder[index] > root:
            index += 1
        return index + 1 == len(postorder) and self.verifyPostorder(left) and self.verifyPostorder(right)


if __name__ == "__main__":
    s = Solution()
    assert s.verifyPostorder([1, 6, 3, 2, 5]) is False
    assert s.verifyPostorder([1, 3, 2, 6, 5]) is True
    assert s.verifyPostorder([1, 2, 3, 4, 5]) is True
    assert s.verifyPostorder([5, 4, 3, 2, 1]) is True
    assert s.verifyPostorder([]) is True
