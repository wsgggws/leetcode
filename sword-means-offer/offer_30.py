from typing import List


class MinStack:
    def __init__(self):
        """
        initialize your data structure here.
        ----top-------
        |(value, min)|
        |(value, min)|
        |(value2, value2 if value2 < value1 else value1)|
        |(value1, value1)|
        """
        self.elems: List[(int, int)] = []
        self.count = -1

    def push(self, x: int) -> None:
        mins = x
        if self.count >= 0 and self.elems[self.count][1] < x:
            mins = self.elems[self.count][1]
        self.elems.append((x, mins))
        self.count += 1

    def pop(self) -> None:
        if self.count >= 0:
            self.elems = self.elems[:-1]
            self.count -= 1

    def top(self) -> int:
        if self.count >= 0:
            return self.elems[self.count][0]

    def min(self) -> int:
        if self.count >= 0:
            return self.elems[self.count][1]


if __name__ == "__main__":
    obj = MinStack()
    assert obj.min() is None
    assert obj.top() is None
    obj.pop()

    minStack = MinStack()
    minStack.push(-2)
    minStack.push(0)
    minStack.push(-3)
    assert minStack.min() == -3
    minStack.pop()
    assert minStack.top() == 0
    assert minStack.min() == -2
