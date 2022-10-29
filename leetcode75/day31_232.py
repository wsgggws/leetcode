class MyQueue:
    """
    stack: 后入先出
    queue: 先入先出

    stack1      stack2

    |   |       |   |
    | 2 |       |   |
    | 1 |       |   |

    push 操作: 往 stack1 里压
    peek 操作:
        1. stack2    空: stack1 所有出栈并依次压入 stack2, 返回 stack2 栈顶
        2. stack2  非空: 直接返回 stack2 栈顶
    pop 操作:
        1. stack2    空: stack1 所有出栈并依次压入 stack2, 弹出 stack2 栈顶
        2. stack2  非空: 直接弹出 stack2 栈顶
    empty:
        stack2 is empty and stack1 is empty
    """

    def __init__(self):
        self.stack1 = []
        self.stack2 = []

    def push(self, x: int) -> None:
        self.stack1.append(x)

    def pop(self) -> int:
        ans = self.peek()
        self.stack2 = self.stack2[:-1]
        return ans

    def peek(self) -> int:
        if len(self.stack2) == 0:
            self.stack2 = self.stack1[::-1]
            self.stack1 = []
        if len(self.stack2) > 0:
            return self.stack2[-1]

    def empty(self) -> bool:
        return len(self.stack1) == len(self.stack2) == 0


if __name__ == "__main__":
    # Your MyQueue object will be instantiated and called as such:
    obj = MyQueue()
    obj.push(1)
    obj.push(2)
    assert obj.peek() == 1
    assert obj.pop() == 1
    assert obj.empty() is False
    assert obj.pop() == 2
    assert obj.pop() is None
    assert obj.empty() is True
