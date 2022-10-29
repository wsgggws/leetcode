from typing import List, Tuple


class MinStack:
    """
    stack
    (-3, -3)
    (0, -2)
    (-2, -2)
    """

    def __init__(self):
        self.stack: List[Tuple[int, int]] = []

    def push(self, val: int) -> None:
        if len(self.stack) == 0:
            self.stack.append((val, val))
        else:
            cur_mins = self.stack[-1][1]
            if val < cur_mins:
                self.stack.append((val, val))
            else:
                self.stack.append((val, cur_mins))

    def pop(self) -> None:
        self.stack = self.stack[:-1]

    def top(self) -> int:
        if self.stack:
            return self.stack[-1][0]

    def getMin(self) -> int:
        if self.stack:
            return self.stack[-1][1]


if __name__ == "__main__":
    # Your MinStack object will be instantiated and called as such:
    obj = MinStack()
    obj.push(-2)
    obj.push(0)
    obj.push(-3)
    assert obj.getMin() == -3
    obj.pop()
    assert obj.top() == 0
    assert obj.getMin() == -2
