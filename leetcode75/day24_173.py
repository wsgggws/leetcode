from typing import List, Optional

from helper import TreeNode, create_binary_tree


class BSTIterator:
    def __init__(self, root: Optional[TreeNode]):
        self.cur: Optional[TreeNode] = root
        self.stack: List[TreeNode] = []

    def next(self) -> int:
        while self.cur:
            self.stack.append(self.cur)
            self.cur = self.cur.left
        self.cur = self.stack.pop()
        val = self.cur.val
        self.cur = self.cur.right
        return val

    def hasNext(self) -> bool:
        return self.cur is not None or len(self.stack) > 0


if __name__ == "__main__":
    root = create_binary_tree([7, 3, 15, None, None, 9, 20], index=0)
    bSTIterator = BSTIterator(root)
    assert bSTIterator.next() == 3
    assert bSTIterator.next() == 7
    assert bSTIterator.hasNext() is True
    assert bSTIterator.next() == 9
    assert bSTIterator.hasNext() is True
    assert bSTIterator.next() == 15
    assert bSTIterator.hasNext() is True
    assert bSTIterator.next() == 20
    assert bSTIterator.hasNext() is False
